// Code generated by protoc-gen-ts_proto. DO NOT EDIT.
// versions:
//   protoc-gen-ts_proto  v2.6.1
//   protoc               v3.6.1
// source: GameEvents.proto

/* eslint-disable */
import { BinaryReader, BinaryWriter } from "@bufbuild/protobuf/wire";

export const protobufPackage = "game_events";

/** Message representing a player joining. */
export interface PlayerJoined {
  id: number;
  name: string;
}

/** Message representing a player leaving. */
export interface PlayerLeft {
  id: number;
}

/** Message representing a player renaming themselves. */
export interface PlayerRename {
  id: number;
  name: string;
}

/** Message representing a player increasing their score. */
export interface PlayerIncreaseScore {
  id: number;
  score: number;
}

/** Union of different event types, modeled as a oneof. */
export interface Event {
  playerJoined?: PlayerJoined | undefined;
  playerLeft?: PlayerLeft | undefined;
  playerRename?: PlayerRename | undefined;
  playerIncreaseScore?: PlayerIncreaseScore | undefined;
}

/** Message representing a list of events. */
export interface EventList {
  events: Event[];
}

function createBasePlayerJoined(): PlayerJoined {
  return { id: 0, name: "" };
}

export const PlayerJoined: MessageFns<PlayerJoined> = {
  encode(message: PlayerJoined, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.id !== 0) {
      writer.uint32(8).uint32(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): PlayerJoined {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerJoined();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 8) {
            break;
          }

          message.id = reader.uint32();
          continue;
        }
        case 2: {
          if (tag !== 18) {
            break;
          }

          message.name = reader.string();
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PlayerJoined {
    return {
      id: isSet(object.id) ? globalThis.Number(object.id) : 0,
      name: isSet(object.name) ? globalThis.String(object.name) : "",
    };
  },

  toJSON(message: PlayerJoined): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<PlayerJoined>, I>>(base?: I): PlayerJoined {
    return PlayerJoined.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<PlayerJoined>, I>>(object: I): PlayerJoined {
    const message = createBasePlayerJoined();
    message.id = object.id ?? 0;
    message.name = object.name ?? "";
    return message;
  },
};

function createBasePlayerLeft(): PlayerLeft {
  return { id: 0 };
}

export const PlayerLeft: MessageFns<PlayerLeft> = {
  encode(message: PlayerLeft, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.id !== 0) {
      writer.uint32(8).uint32(message.id);
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): PlayerLeft {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerLeft();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 8) {
            break;
          }

          message.id = reader.uint32();
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PlayerLeft {
    return { id: isSet(object.id) ? globalThis.Number(object.id) : 0 };
  },

  toJSON(message: PlayerLeft): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<PlayerLeft>, I>>(base?: I): PlayerLeft {
    return PlayerLeft.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<PlayerLeft>, I>>(object: I): PlayerLeft {
    const message = createBasePlayerLeft();
    message.id = object.id ?? 0;
    return message;
  },
};

function createBasePlayerRename(): PlayerRename {
  return { id: 0, name: "" };
}

export const PlayerRename: MessageFns<PlayerRename> = {
  encode(message: PlayerRename, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.id !== 0) {
      writer.uint32(8).uint32(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): PlayerRename {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerRename();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 8) {
            break;
          }

          message.id = reader.uint32();
          continue;
        }
        case 2: {
          if (tag !== 18) {
            break;
          }

          message.name = reader.string();
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PlayerRename {
    return {
      id: isSet(object.id) ? globalThis.Number(object.id) : 0,
      name: isSet(object.name) ? globalThis.String(object.name) : "",
    };
  },

  toJSON(message: PlayerRename): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<PlayerRename>, I>>(base?: I): PlayerRename {
    return PlayerRename.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<PlayerRename>, I>>(object: I): PlayerRename {
    const message = createBasePlayerRename();
    message.id = object.id ?? 0;
    message.name = object.name ?? "";
    return message;
  },
};

function createBasePlayerIncreaseScore(): PlayerIncreaseScore {
  return { id: 0, score: 0 };
}

export const PlayerIncreaseScore: MessageFns<PlayerIncreaseScore> = {
  encode(message: PlayerIncreaseScore, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.id !== 0) {
      writer.uint32(8).uint32(message.id);
    }
    if (message.score !== 0) {
      writer.uint32(16).uint32(message.score);
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): PlayerIncreaseScore {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerIncreaseScore();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 8) {
            break;
          }

          message.id = reader.uint32();
          continue;
        }
        case 2: {
          if (tag !== 16) {
            break;
          }

          message.score = reader.uint32();
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PlayerIncreaseScore {
    return {
      id: isSet(object.id) ? globalThis.Number(object.id) : 0,
      score: isSet(object.score) ? globalThis.Number(object.score) : 0,
    };
  },

  toJSON(message: PlayerIncreaseScore): unknown {
    const obj: any = {};
    if (message.id !== 0) {
      obj.id = Math.round(message.id);
    }
    if (message.score !== 0) {
      obj.score = Math.round(message.score);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<PlayerIncreaseScore>, I>>(base?: I): PlayerIncreaseScore {
    return PlayerIncreaseScore.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<PlayerIncreaseScore>, I>>(object: I): PlayerIncreaseScore {
    const message = createBasePlayerIncreaseScore();
    message.id = object.id ?? 0;
    message.score = object.score ?? 0;
    return message;
  },
};

function createBaseEvent(): Event {
  return { playerJoined: undefined, playerLeft: undefined, playerRename: undefined, playerIncreaseScore: undefined };
}

export const Event: MessageFns<Event> = {
  encode(message: Event, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.playerJoined !== undefined) {
      PlayerJoined.encode(message.playerJoined, writer.uint32(10).fork()).join();
    }
    if (message.playerLeft !== undefined) {
      PlayerLeft.encode(message.playerLeft, writer.uint32(18).fork()).join();
    }
    if (message.playerRename !== undefined) {
      PlayerRename.encode(message.playerRename, writer.uint32(26).fork()).join();
    }
    if (message.playerIncreaseScore !== undefined) {
      PlayerIncreaseScore.encode(message.playerIncreaseScore, writer.uint32(34).fork()).join();
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): Event {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEvent();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 10) {
            break;
          }

          message.playerJoined = PlayerJoined.decode(reader, reader.uint32());
          continue;
        }
        case 2: {
          if (tag !== 18) {
            break;
          }

          message.playerLeft = PlayerLeft.decode(reader, reader.uint32());
          continue;
        }
        case 3: {
          if (tag !== 26) {
            break;
          }

          message.playerRename = PlayerRename.decode(reader, reader.uint32());
          continue;
        }
        case 4: {
          if (tag !== 34) {
            break;
          }

          message.playerIncreaseScore = PlayerIncreaseScore.decode(reader, reader.uint32());
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Event {
    return {
      playerJoined: isSet(object.playerJoined) ? PlayerJoined.fromJSON(object.playerJoined) : undefined,
      playerLeft: isSet(object.playerLeft) ? PlayerLeft.fromJSON(object.playerLeft) : undefined,
      playerRename: isSet(object.playerRename) ? PlayerRename.fromJSON(object.playerRename) : undefined,
      playerIncreaseScore: isSet(object.playerIncreaseScore)
        ? PlayerIncreaseScore.fromJSON(object.playerIncreaseScore)
        : undefined,
    };
  },

  toJSON(message: Event): unknown {
    const obj: any = {};
    if (message.playerJoined !== undefined) {
      obj.playerJoined = PlayerJoined.toJSON(message.playerJoined);
    }
    if (message.playerLeft !== undefined) {
      obj.playerLeft = PlayerLeft.toJSON(message.playerLeft);
    }
    if (message.playerRename !== undefined) {
      obj.playerRename = PlayerRename.toJSON(message.playerRename);
    }
    if (message.playerIncreaseScore !== undefined) {
      obj.playerIncreaseScore = PlayerIncreaseScore.toJSON(message.playerIncreaseScore);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Event>, I>>(base?: I): Event {
    return Event.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Event>, I>>(object: I): Event {
    const message = createBaseEvent();
    message.playerJoined = (object.playerJoined !== undefined && object.playerJoined !== null)
      ? PlayerJoined.fromPartial(object.playerJoined)
      : undefined;
    message.playerLeft = (object.playerLeft !== undefined && object.playerLeft !== null)
      ? PlayerLeft.fromPartial(object.playerLeft)
      : undefined;
    message.playerRename = (object.playerRename !== undefined && object.playerRename !== null)
      ? PlayerRename.fromPartial(object.playerRename)
      : undefined;
    message.playerIncreaseScore = (object.playerIncreaseScore !== undefined && object.playerIncreaseScore !== null)
      ? PlayerIncreaseScore.fromPartial(object.playerIncreaseScore)
      : undefined;
    return message;
  },
};

function createBaseEventList(): EventList {
  return { events: [] };
}

export const EventList: MessageFns<EventList> = {
  encode(message: EventList, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    for (const v of message.events) {
      Event.encode(v!, writer.uint32(10).fork()).join();
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): EventList {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEventList();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1: {
          if (tag !== 10) {
            break;
          }

          message.events.push(Event.decode(reader, reader.uint32()));
          continue;
        }
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): EventList {
    return { events: globalThis.Array.isArray(object?.events) ? object.events.map((e: any) => Event.fromJSON(e)) : [] };
  },

  toJSON(message: EventList): unknown {
    const obj: any = {};
    if (message.events?.length) {
      obj.events = message.events.map((e) => Event.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<EventList>, I>>(base?: I): EventList {
    return EventList.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<EventList>, I>>(object: I): EventList {
    const message = createBaseEventList();
    message.events = object.events?.map((e) => Event.fromPartial(e)) || [];
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

export interface MessageFns<T> {
  encode(message: T, writer?: BinaryWriter): BinaryWriter;
  decode(input: BinaryReader | Uint8Array, length?: number): T;
  fromJSON(object: any): T;
  toJSON(message: T): unknown;
  create<I extends Exact<DeepPartial<T>, I>>(base?: I): T;
  fromPartial<I extends Exact<DeepPartial<T>, I>>(object: I): T;
}
