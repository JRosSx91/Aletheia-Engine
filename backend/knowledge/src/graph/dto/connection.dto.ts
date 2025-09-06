// src/graph/dto/connection.dto.ts
import { IsString, IsUUID, ValidateNested } from 'class-validator';
import { Type } from 'class-transformer';
import { NodeDto } from './node.dto';
import { Expose } from 'class-transformer';
export class ConnectionDto {
  @Expose()
  @IsUUID()
  id: string;

  @Expose()
  @IsString()
  type: string;

  @Expose()
  @ValidateNested()
  @Type(() => NodeDto)
  fromNode: NodeDto;

  @Expose()
  @ValidateNested()
  @Type(() => NodeDto)
  toNode: NodeDto;
}
