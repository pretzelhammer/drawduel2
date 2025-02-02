// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

export class PlayerLeave {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):PlayerLeave {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsPlayerLeave(bb:flatbuffers.ByteBuffer, obj?:PlayerLeave):PlayerLeave {
  return (obj || new PlayerLeave()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsPlayerLeave(bb:flatbuffers.ByteBuffer, obj?:PlayerLeave):PlayerLeave {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new PlayerLeave()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

id():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

static startPlayerLeave(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addId(builder:flatbuffers.Builder, id:number) {
  builder.addFieldInt16(0, id, 0);
}

static endPlayerLeave(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createPlayerLeave(builder:flatbuffers.Builder, id:number):flatbuffers.Offset {
  PlayerLeave.startPlayerLeave(builder);
  PlayerLeave.addId(builder, id);
  return PlayerLeave.endPlayerLeave(builder);
}
}
