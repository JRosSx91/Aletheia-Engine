import { SetMetadata } from '@nestjs/common';

export interface ClassConstructor {
  new (...args: any[]): object;
}

export const SERIALIZE_DTO_KEY = 'serialize_dto';

export const Serialize = (dto: ClassConstructor) =>
  SetMetadata(SERIALIZE_DTO_KEY, dto);
