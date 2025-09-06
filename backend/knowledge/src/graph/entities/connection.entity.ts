import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  ManyToOne,
  CreateDateColumn,
} from 'typeorm';
import { Node } from './node.entity';

@Entity('connections')
export class Connection {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ type: 'varchar' })
  type: string;

  @ManyToOne(() => Node, { eager: true })
  fromNode: Node;

  @ManyToOne(() => Node, { eager: true })
  toNode: Node;

  @CreateDateColumn()
  createdAt: Date;
}
