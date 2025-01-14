// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

export class PlayerIncreaseScore {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):PlayerIncreaseScore {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsPlayerIncreaseScore(bb:flatbuffers.ByteBuffer, obj?:PlayerIncreaseScore):PlayerIncreaseScore {
  return (obj || new PlayerIncreaseScore()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsPlayerIncreaseScore(bb:flatbuffers.ByteBuffer, obj?:PlayerIncreaseScore):PlayerIncreaseScore {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new PlayerIncreaseScore()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

id():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint32(this.bb_pos + offset) : 0;
}

score():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint32(this.bb_pos + offset) : 0;
}

static startPlayerIncreaseScore(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addId(builder:flatbuffers.Builder, id:number) {
  builder.addFieldInt32(0, id, 0);
}

static addScore(builder:flatbuffers.Builder, score:number) {
  builder.addFieldInt32(1, score, 0);
}

static endPlayerIncreaseScore(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createPlayerIncreaseScore(builder:flatbuffers.Builder, id:number, score:number):flatbuffers.Offset {
  PlayerIncreaseScore.startPlayerIncreaseScore(builder);
  PlayerIncreaseScore.addId(builder, id);
  PlayerIncreaseScore.addScore(builder, score);
  return PlayerIncreaseScore.endPlayerIncreaseScore(builder);
}
}