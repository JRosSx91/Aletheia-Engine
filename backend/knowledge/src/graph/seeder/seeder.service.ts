import { Injectable, OnModuleInit } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Node } from '../entities/node.entity';
import { Connection } from '../entities/connection.entity';

@Injectable()
export class SeederService implements OnModuleInit {
  constructor(
    @InjectRepository(Node)
    private readonly nodeRepository: Repository<Node>,
    @InjectRepository(Connection)
    private readonly connectionRepository: Repository<Connection>,
  ) {}

  async onModuleInit() {
    console.log('Seeder: Starting data seeding process...');
    await this.seed();
    console.log('Seeder: Data seeding process completed.');
  }

  private async seed() {
    const nodeCount = await this.nodeRepository.count();
    if (nodeCount > 0) {
      console.log(
        'Seeder: The database already contains data. No seeding is needed.',
      );
      return;
    }

    // 2. Crear los Nodos (los conceptos)
    console.log('Seeder: Creating nodes...');
    const nodePlanetDef = await this.nodeRepository.save({
      title: "Definition of 'Planet' (UAI 2006)",
    });
    const nodeCritery1 = await this.nodeRepository.save({
      title: 'Criterion 1: Orbiting a star (The Sun)',
    });
    const nodeCritery2 = await this.nodeRepository.save({
      title: 'Criterion 2: Reaching Hydrostatic Equilibrium',
    });
    const nodeCritery3 = await this.nodeRepository.save({
      title: 'Criterion 3: Having cleaned its neighborhood of its orbit',
    });
    const nodePluton = await this.nodeRepository.save({ title: 'Pluton' });
    const nodeKuiperBelt = await this.nodeRepository.save({
      title: 'Kuiper Belt',
    });
    const nodeOrbitCleaningExplanation = await this.nodeRepository.save({
      title: "'Orbit Cleaning' (Explanation)",
    });

    // 3. Crear las Conexiones (las relaciones lógicas)
    console.log('Seeder: Creating connections...');
    await this.connectionRepository.save([
      // La Definición de Planeta REQUIERE los 3 criterios
      { fromNode: nodePlanetDef, toNode: nodeCritery1, type: 'REQUIRES' },
      { fromNode: nodePlanetDef, toNode: nodeCritery2, type: 'REQUIRES' },
      { fromNode: nodePlanetDef, toNode: nodeCritery3, type: 'REQUIRES' },

      // Plutón CUMPLE los dos primeros criterios
      {
        fromNode: nodePluton,
        toNode: nodeCritery1,
        type: 'EVALUATES_AND_SATISFIES',
      },
      {
        fromNode: nodePluton,
        toNode: nodeCritery2,
        type: 'EVALUATES_AND_SATISFIES',
      },

      // Plutón NO CUMPLE el tercer criterio
      {
        fromNode: nodePluton,
        toNode: nodeCritery3,
        type: 'EVALUATES_AND_NO_SATISFIES',
      },

      // El Criterio 3 se explica con el concepto de 'Limpiar la órbita'
      {
        fromNode: nodeCritery3,
        toNode: nodeOrbitCleaningExplanation,
        type: 'EXPLAINS_WITH_EXAMPLE',
      },

      // La explicación de 'Limpiar la órbita' usa como ejemplo el Cinturón de Kuiper
      {
        fromNode: nodeOrbitCleaningExplanation,
        toNode: nodeKuiperBelt,
        type: 'USES_EXAMPLE',
      },
    ]);
  }
}
