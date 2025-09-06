import { Controller, Get, UseInterceptors } from '@nestjs/common';
import { GraphService } from './graph.service';
import { GraphDto } from '../dto/graph.dto';
import { TransformInterceptor } from '../../interceptors/transform.interceptor';
import { Serialize } from '../../decorators/serialize.decorator';

@Controller('graph')
export class GraphController {
  constructor(private readonly graphService: GraphService) {}

  @Get()
  @UseInterceptors(TransformInterceptor)
  @Serialize(GraphDto)
  findFullGraph(): Promise<any> {
    console.log('GraphController: Petition received in GET /graph');
    return this.graphService.getFullGraph();
  }
}
