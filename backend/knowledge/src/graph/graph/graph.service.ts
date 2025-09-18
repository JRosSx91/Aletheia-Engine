import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Node } from '../entities/node.entity';
import { Connection } from '../entities/connection.entity';

@Injectable()
export class GraphService {
  constructor(
    @InjectRepository(Node)
    private readonly nodeRepository: Repository<Node>,
    @InjectRepository(Connection)
    private readonly connectionRepository: Repository<Connection>,
  ) {}

  async getFullGraph() {
    console.log('GraphService: Returning pure entities...');
    const nodes = await this.nodeRepository.find();
    const connections = await this.connectionRepository.find({
      relations: ['fromNode', 'toNode'],
    });

    return { nodes, connections };
  }
}
