import {
  BufferGeometry,
  DynamicDrawUsage,
  Float32BufferAttribute,
  MathUtils,
  Uint32BufferAttribute,
  Vector3,
} from "three";
import { SimplexNoise } from "./SimplexNoise.js";


      return this.subrays[this.numSubrays++];
    }

    initSubray(subray, rayParameters) {
      subray.pos0.copy(rayParameters.sourceOffset);
      subray.pos1.copy(rayParameters.destOffset);
      subray.up0.copy(rayParameters.up0);
      subray.up1.copy(rayParameters.up1);
      subray.radius0 = rayParameters.radius0;
      subray.radius1 = rayParameters.radius1;
      subray.birthTime = rayParameters.birthTime;
      subray.deathTime = rayParameters.deathTime;
      subray.timeScale = rayParameters.timeScale;
      subray.roughness = rayParameters.roughness;
      subray.straightness = rayParameters.straightness;
      subray.propagationTimeFactor = rayParameters.propagationTimeFactor;
      subray.vanishingTimeFactor = rayParameters.vanishingTimeFactor;

      subray.maxIterations = this.maxIterations;
      subray.seed =
        rayParameters.noiseSeed !== undefined ? rayParameters.noiseSeed : 0;
      subray.recursion = 0;
    }

    fractalRay(time, segmentCallback) {
      this.time = time;
      this.currentSegmentCallback = segmentCallback;
      this.numSubrays = 0;

      this.initSubray(this.addNewSubray(), this.rayParameters);

      for (let subrayIndex = 0; subrayIndex < this.numSubrays; subrayIndex++) {
        const subray = this.subrays[subrayIndex];
        this.currentSubray = subray;

        this.randomGenerator.setSeed(subray.seed);

        subray.endPropagationTime = MathUtils.lerp(
          subray.birthTime,
          subray.deathTime,
          subray.propagationTimeFactor
        );
        subray.beginVanishingTime = MathUtils.lerp(
          subray.deathTime,
          subray.birthTime,
          1 - subray.vanishingTimeFactor
        );

        const random1 = this.randomGenerator.random;
        subray.linPos0
          .set(random1(), random1(), random1())
          .multiplyScalar(1000);
        subray.linPos1
          .set(random1(), random1(), random1())
          .multiplyScalar(1000);

        this.timeFraction =
          (time - subray.birthTime) / (subray.deathTime - subray.birthTime);

        this.currentSegmentIndex = 0;
        this.isInitialSegment = true;

        const segment = this.getNewSegment();
        segment.iteration = 0;
        segment.pos0.copy(subray.pos0);
        segment.pos1.copy(subray.pos1);
        segment.linPos0.copy(subray.linPos0);
        segment.linPos1.copy(subray.linPos1);
        segment.up0.copy(subray.up0);
        segment.up1.copy(subray.up1);
        segment.radius0 = subray.radius0;
        segment.radius1 = subray.radius1;
        segment.fraction0 = 0;
        segment.fraction1 = 1;
        segment.positionVariationFactor = 1 - subray.straightness;

        this.subrayProbability =
          (this.ramification *
            Math.pow(this.recursionProbability, subray.recursion)) /
          (1 << subray.maxIterations);

        this.fractalRayRecursive(segment);
      }

      this.currentSegmentCallback = null;
      this.currentSubray = null;
    }

    fractalRayRecursive(segment) {
      if (segment.iteration >= this.currentSubray.maxIterations) {
        this.currentSegmentCallback(segment);

        return;
      }

      this.forwards.subVectors(segment.pos1, segment.pos0);
      let lForwards = this.forwards.length();

      if (lForwards < 0.000001) {
        this.forwards.set(0, 0, 0.01);
        lForwards = this.forwards.length();
      }

      const middleRadius = (segment.radius0 + segment.radius1) * 0.5;
      const middleFraction = (segment.fraction0 + segment.fraction1) * 0.5;

      const timeDimension =
        this.time *
        this.currentSubray.timeScale *
        Math.pow(2, segment.iteration);

      this.middlePos.lerpVectors(segment.pos0, segment.pos1, 0.5);
      this.middleLinPos.lerpVectors(segment.linPos0, segment.linPos1, 0.5);
      const p = this.middleLinPos;

      this.newPos.set(
        this.simplexX.noise4d(p.x, p.y, p.z, timeDimension),
        this.simplexY.noise4d(p.x, p.y, p.z, timeDimension),
        this.simplexZ.noise4d(p.x, p.y, p.z, timeDimension)
      );

      this.newPos.multiplyScalar(segment.positionVariationFactor * lForwards);
      this.newPos.add(this.middlePos);


      const newSegment1 = this.getNewSegment();
      newSegment1.pos0.copy(segment.pos0);
      newSegment1.pos1.copy(this.newPos);
      newSegment1.linPos0.copy(segment.linPos0);
      newSegment1.linPos1.copy(this.middleLinPos);
      newSegment1.up0.copy(segment.up0);
      newSegment1.up1.copy(segment.up1);
      newSegment1.radius0 = segment.radius0;
      newSegment1.radius1 = middleRadius;
      newSegment1.fraction0 = segment.fraction0;
      newSegment1.fraction1 = middleFraction;
      newSegment1.positionVariationFactor =
        segment.positionVariationFactor * this.currentSubray.roughness;
      newSegment1.iteration = segment.iteration + 1;

      const newSegment2 = this.getNewSegment();
      newSegment2.pos0.copy(this.newPos);
      newSegment2.pos1.copy(segment.pos1);
      newSegment2.linPos0.copy(this.middleLinPos);
      newSegment2.linPos1.copy(segment.linPos1);
      this.cross1.crossVectors(segment.up0, this.forwards.normalize());
      newSegment2.up0.crossVectors(this.forwards, this.cross1).normalize();
      newSegment2.up1.copy(segment.up1);
      newSegment2.radius0 = middleRadius;
      newSegment2.radius1 = segment.radius1;
      newSegment2.fraction0 = middleFraction;
      newSegment2.fraction1 = segment.fraction1;
      newSegment2.positionVariationFactor =
        segment.positionVariationFactor * this.currentSubray.roughness;
      newSegment2.iteration = segment.iteration + 1;

      this.fractalRayRecursive(newSegment1);

      this.fractalRayRecursive(newSegment2);
    }

    createPrism(segment) {

      this.forwardsFill.subVectors(segment.pos1, segment.pos0).normalize();

      if (this.isInitialSegment) {
        this.currentCreateTriangleVertices(
          segment.pos0,
          segment.up0,
          this.forwardsFill,
          segment.radius0,
          0
        );

        this.isInitialSegment = false;
      }

      this.currentCreateTriangleVertices(
        segment.pos1,
        segment.up0,
        this.forwardsFill,
        segment.radius1,
        segment.fraction1
      );

      this.createPrismFaces();
    }

    createTriangleVerticesWithoutUVs(pos, up, forwards, radius) {

      this.side
        .crossVectors(up, forwards)
        .multiplyScalar(radius * LightningStrike.COS30DEG);
      this.down.copy(up).multiplyScalar(-radius * LightningStrike.SIN30DEG);

      const p = this.vPos;
      const v = this.vertices;

      p.copy(pos).sub(this.side).add(this.down);

      v[this.currentCoordinate++] = p.x;
      v[this.currentCoordinate++] = p.y;
      v[this.currentCoordinate++] = p.z;

      p.copy(pos).add(this.side).add(this.down);

      v[this.currentCoordinate++] = p.x;
      v[this.currentCoordinate++] = p.y;
      v[this.currentCoordinate++] = p.z;

      p.copy(up).multiplyScalar(radius).add(pos);

      v[this.currentCoordinate++] = p.x;
      v[this.currentCoordinate++] = p.y;
      v[this.currentCoordinate++] = p.z;

      this.currentVertex += 3;
    }

    createTriangleVerticesWithUVs(pos, up, forwards, radius, u) {

      this.side
        .crossVectors(up, forwards)
        .multiplyScalar(radius * LightningStrike.COS30DEG);
      this.down.copy(up).multiplyScalar(-radius * LightningStrike.SIN30DEG);

      const p = this.vPos;
      const v = this.vertices;
      const uv = this.uvs;

      p.copy(pos).sub(this.side).add(this.down);

      v[this.currentCoordinate++] = p.x;
      v[this.currentCoordinate++] = p.y;
      v[this.currentCoordinate++] = p.z;

      uv[this.currentUVCoordinate++] = u;
      uv[this.currentUVCoordinate++] = 0;

      p.copy(pos).add(this.side).add(this.down);

      v[this.currentCoordinate++] = p.x;
      v[this.currentCoordinate++] = p.y;
      v[this.currentCoordinate++] = p.z;

      uv[this.currentUVCoordinate++] = u;
      uv[this.currentUVCoordinate++] = 0.5;

      p.copy(up).multiplyScalar(radius).add(pos);

      v[this.currentCoordinate++] = p.x;
      v[this.currentCoordinate++] = p.y;
      v[this.currentCoordinate++] = p.z;

      uv[this.currentUVCoordinate++] = u;
      uv[this.currentUVCoordinate++] = 1;

      this.currentVertex += 3;
    }

