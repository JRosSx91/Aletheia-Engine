import {
  CallHandler,
  ExecutionContext,
  Injectable,
  NestInterceptor,
} from '@nestjs/common';
import { Reflector } from '@nestjs/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { plainToInstance } from 'class-transformer';
import {
  SERIALIZE_DTO_KEY,
  ClassConstructor,
} from '../decorators/serialize.decorator';

@Injectable()
export class TransformInterceptor implements NestInterceptor {
  constructor(private readonly reflector: Reflector) {}

  intercept(context: ExecutionContext, next: CallHandler): Observable<any> {
    const dto = this.reflector.get<ClassConstructor>(
      SERIALIZE_DTO_KEY,
      context.getHandler(),
    );
    if (!dto) {
      return next.handle();
    }

    return next.handle().pipe(
      map((data) => {
        return plainToInstance(dto, data, {
          excludeExtraneousValues: true,
        });
      }),
    );
  }
}
