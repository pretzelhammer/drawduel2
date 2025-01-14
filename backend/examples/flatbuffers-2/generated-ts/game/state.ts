// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { Player } from '../game/player.js';


export class State {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):State {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsState(bb:flatbuffers.ByteBuffer, obj?:State):State {
  return (obj || new State()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsState(bb:flatbuffers.ByteBuffer, obj?:State):State {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new State()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

players(index: number, obj?:Player):Player|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new Player()).__init(this.bb!.__indirect(this.bb!.__vector(this.bb_pos + offset) + index * 4), this.bb!) : null;
}

playersLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

admin():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint32(this.bb_pos + offset) : 0;
}

static startState(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addPlayers(builder:flatbuffers.Builder, playersOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, playersOffset, 0);
}

static createPlayersVector(builder:flatbuffers.Builder, data:flatbuffers.Offset[]):flatbuffers.Offset {
  builder.startVector(4, data.length, 4);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addOffset(data[i]!);
  }
  return builder.endVector();
}

static startPlayersVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(4, numElems, 4);
}

static addAdmin(builder:flatbuffers.Builder, admin:number) {
  builder.addFieldInt32(1, admin, 0);
}

static endState(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static finishStateBuffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset);
}

static finishSizePrefixedStateBuffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset, undefined, true);
}

static createState(builder:flatbuffers.Builder, playersOffset:flatbuffers.Offset, admin:number):flatbuffers.Offset {
  State.startState(builder);
  State.addPlayers(builder, playersOffset);
  State.addAdmin(builder, admin);
  return State.endState(builder);
}
}
