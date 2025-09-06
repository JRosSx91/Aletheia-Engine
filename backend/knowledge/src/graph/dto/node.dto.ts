import { IsString, IsUUID, IsOptional } from 'class-validator';
import { Expose } from 'class-transformer';

export class NodeDto {
  @Expose()
  @IsUUID()
  id: string;

  @Expose()
  @IsString()
  title: string;

  @Expose()
  @IsString()
  @IsOptional()
  description?: string;
}
