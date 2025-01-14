// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { EventUnion, unionToEventUnion, unionListToEventUnion } from '../client-event/event-union.js';


export class Event {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):Event {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsEvent(bb:flatbuffers.ByteBuffer, obj?:Event):Event {
  return (obj || new Event()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsEvent(bb:flatbuffers.ByteBuffer, obj?:Event):Event {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new Event()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

eventType():EventUnion {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : EventUnion.NONE;
}

event<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startEvent(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addEventType(builder:flatbuffers.Builder, eventType:EventUnion) {
  builder.addFieldInt8(0, eventType, EventUnion.NONE);
}

static addEvent(builder:flatbuffers.Builder, eventOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, eventOffset, 0);
}

static endEvent(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createEvent(builder:flatbuffers.Builder, eventType:EventUnion, eventOffset:flatbuffers.Offset):flatbuffers.Offset {
  Event.startEvent(builder);
  Event.addEventType(builder, eventType);
  Event.addEvent(builder, eventOffset);
  return Event.endEvent(builder);
}
}
