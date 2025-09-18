import { MathUtils, Mesh, MeshBasicMaterial, Object3D } from "three";
import { LightningStrike } from "./LightningStrike.js";


class LightningStorm extends Object3D {
  constructor(stormParams = {}) {
    super();

    this.isLightningStorm = true;


    this.stormParams = stormParams;

    stormParams.size =
      stormParams.size !== undefined ? stormParams.size : 1000.0;
    stormParams.minHeight =
      stormParams.minHeight !== undefined ? stormParams.minHeight : 80.0;
    stormParams.maxHeight =
      stormParams.maxHeight !== undefined ? stormParams.maxHeight : 100.0;
    stormParams.maxSlope =
      stormParams.maxSlope !== undefined ? stormParams.maxSlope : 1.1;

    stormParams.maxLightnings =
      stormParams.maxLightnings !== undefined ? stormParams.maxLightnings : 3;

    stormParams.lightningMinPeriod =
      stormParams.lightningMinPeriod !== undefined
        ? stormParams.lightningMinPeriod
        : 3.0;
    stormParams.lightningMaxPeriod =
      stormParams.lightningMaxPeriod !== undefined
        ? stormParams.lightningMaxPeriod
        : 7.0;

    stormParams.lightningMinDuration =
      stormParams.lightningMinDuration !== undefined
        ? stormParams.lightningMinDuration
        : 1.0;
    stormParams.lightningMaxDuration =
      stormParams.lightningMaxDuration !== undefined
        ? stormParams.lightningMaxDuration
        : 2.5;

    this.lightningParameters = LightningStrike.copyParameters(
      stormParams.lightningParameters,
      stormParams.lightningParameters
    );

    this.lightningParameters.isEternal = false;

    this.lightningMaterial =
      stormParams.lightningMaterial !== undefined
        ? stormParams.lightningMaterial
        : new MeshBasicMaterial({ color: 0xb0ffff });

    if (stormParams.onRayPosition !== undefined) {
      this.onRayPosition = stormParams.onRayPosition;
    } else {
      this.onRayPosition = function (source, dest) {
        dest.set(
          (Math.random() - 0.5) * stormParams.size,
          0,
          (Math.random() - 0.5) * stormParams.size
        );

        const height = MathUtils.lerp(
          stormParams.minHeight,
          stormParams.maxHeight,
          Math.random()
        );

        source
          .set(
            stormParams.maxSlope * (2 * Math.random() - 1),
            1,
            stormParams.maxSlope * (2 * Math.random() - 1)
          )
          .multiplyScalar(height)
          .add(dest);
      };
    }

    this.onLightningDown = stormParams.onLightningDown;


    this.inited = false;
    this.nextLightningTime = 0;
    this.lightningsMeshes = [];
    this.deadLightningsMeshes = [];

    for (let i = 0; i < this.stormParams.maxLightnings; i++) {
      const lightning = new LightningStrike(
        LightningStrike.copyParameters({}, this.lightningParameters)
      );
      const mesh = new Mesh(lightning, this.lightningMaterial);
      this.deadLightningsMeshes.push(mesh);
    }
  }

  update(time) {
    if (!this.inited) {
      this.nextLightningTime = this.getNextLightningTime(time) * Math.random();
      this.inited = true;
    }

    if (time >= this.nextLightningTime) {

      const lightningMesh = this.deadLightningsMeshes.pop();

      if (lightningMesh) {
        const lightningParams1 = LightningStrike.copyParameters(
          lightningMesh.geometry.rayParameters,
          this.lightningParameters
        );

        lightningParams1.birthTime = time;
        lightningParams1.deathTime =
          time +
          MathUtils.lerp(
            this.stormParams.lightningMinDuration,
            this.stormParams.lightningMaxDuration,
            Math.random()
          );

        this.onRayPosition(
          lightningParams1.sourceOffset,
          lightningParams1.destOffset
        );

        lightningParams1.noiseSeed = Math.random();

        this.add(lightningMesh);

        this.lightningsMeshes.push(lightningMesh);
      }

      this.nextLightningTime = this.getNextLightningTime(time);
    }

    let i = 0,
      il = this.lightningsMeshes.length;

    while (i < il) {
      const mesh = this.lightningsMeshes[i];

      const lightning = mesh.geometry;

      const prevState = lightning.state;

      lightning.update(time);

      if (
        prevState === LightningStrike.RAY_PROPAGATING &&
        lightning.state > prevState
      ) {
        if (this.onLightningDown) {
          this.onLightningDown(lightning);
        }
      }

      if (lightning.state === LightningStrike.RAY_EXTINGUISHED) {

        this.lightningsMeshes.splice(this.lightningsMeshes.indexOf(mesh), 1);

        this.deadLightningsMeshes.push(mesh);

        this.remove(mesh);

        il--;
      } else {
        i++;
      }
    }
  }

  getNextLightningTime(currentTime) {
    return (
      currentTime +
      MathUtils.lerp(
        this.stormParams.lightningMinPeriod,
        this.stormParams.lightningMaxPeriod,
        Math.random()
      ) /
        (this.stormParams.maxLightnings + 1)
    );
  }

  copy(source, recursive) {
    super.copy(source, recursive);

    this.stormParams.size = source.stormParams.size;
    this.stormParams.minHeight = source.stormParams.minHeight;
    this.stormParams.maxHeight = source.stormParams.maxHeight;
    this.stormParams.maxSlope = source.stormParams.maxSlope;

    this.stormParams.maxLightnings = source.stormParams.maxLightnings;

    this.stormParams.lightningMinPeriod = source.stormParams.lightningMinPeriod;
    this.stormParams.lightningMaxPeriod = source.stormParams.lightningMaxPeriod;

    this.stormParams.lightningMinDuration =
      source.stormParams.lightningMinDuration;
    this.stormParams.lightningMaxDuration =
      source.stormParams.lightningMaxDuration;

    this.lightningParameters = LightningStrike.copyParameters(
      {},
      source.lightningParameters
    );

    this.lightningMaterial = source.stormParams.lightningMaterial;

    this.onLightningDown = source.onLightningDown;

    return this;
  }

  clone() {
    return new this.constructor(this.stormParams).copy(this);
  }
}

export { LightningStorm };
