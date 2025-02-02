// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

export class PlayerJoined {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):PlayerJoined {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsPlayerJoined(bb:flatbuffers.ByteBuffer, obj?:PlayerJoined):PlayerJoined {
  return (obj || new PlayerJoined()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsPlayerJoined(bb:flatbuffers.ByteBuffer, obj?:PlayerJoined):PlayerJoined {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new PlayerJoined()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

id():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint32(this.bb_pos + offset) : 0;
}

name():string|null
name(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
name(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startPlayerJoined(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addId(builder:flatbuffers.Builder, id:number) {
  builder.addFieldInt32(0, id, 0);
}

static addName(builder:flatbuffers.Builder, nameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, nameOffset, 0);
}

static endPlayerJoined(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createPlayerJoined(builder:flatbuffers.Builder, id:number, nameOffset:flatbuffers.Offset):flatbuffers.Offset {
  PlayerJoined.startPlayerJoined(builder);
  PlayerJoined.addId(builder, id);
  PlayerJoined.addName(builder, nameOffset);
  return PlayerJoined.endPlayerJoined(builder);
}
}
