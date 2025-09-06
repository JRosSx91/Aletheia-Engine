import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Node } from './entities/node.entity';
import { Connection } from './entities/connection.entity';
import { SeederService } from './seeder/seeder.service';
import { GraphService } from './graph/graph.service';
import { GraphController } from './graph/graph.controller';

@Module({
  imports: [TypeOrmModule.forFeature([Node, Connection])],
  providers: [SeederService, GraphService],
  controllers: [GraphController],
})
export class GraphModule {}
