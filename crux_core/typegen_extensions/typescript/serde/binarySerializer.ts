/**
 * Copyright (c) Facebook, Inc. and its affiliates
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

import { Serializer } from "./serializer";

export abstract class BinarySerializer implements Serializer {
  private static readonly BIG_32: bigint = BigInt(32);
  private static readonly BIG_64: bigint = BigInt(64);

  private static readonly BIG_32Fs: bigint = BigInt("4294967295");
  private static readonly BIG_64Fs: bigint = BigInt("18446744073709551615");

  private static readonly textEncoder = new TextEncoder();

  private buffer: ArrayBuffer;
  private offset: number;

  constructor() {
    this.buffer = new ArrayBuffer(64);
    this.offset = 0;
  }

  private ensureBufferWillHandleSize(bytes: number) {
    while (this.buffer.byteLength < this.offset + bytes) {
      const newBuffer = new ArrayBuffer(this.buffer.byteLength * 2);
      new Uint8Array(newBuffer).set(new Uint8Array(this.buffer));
      this.buffer = newBuffer;
    }
  }

  protected serialize(values: Uint8Array) {
    this.ensureBufferWillHandleSize(values.length);
    new Uint8Array(this.buffer, this.offset).set(values);
    this.offset += values.length;
  }

  abstract serializeLen(value: number): void;

  abstract serializeVariantIndex(value: number): void;

  abstract sortMapEntries(offsets: number[]): void;

  public serializeStr(value: string): void {
    this.serializeBytes(BinarySerializer.textEncoder.encode(value));
  }

  public serialize