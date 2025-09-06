import { ValidateNested } from 'class-validator';
import { Expose, Type } from 'class-transformer';
import { NodeDto } from './node.dto';
import { ConnectionDto } from './connection.dto';

export class GraphDto {
  @Expose()
  @ValidateNested({ each: true })
  @Type(() => NodeDto)
  nodes: NodeDto[];

  @Expose()
  @ValidateNested({ each: true })
  @Type(() => ConnectionDto)
  connections: ConnectionDto[];
}
