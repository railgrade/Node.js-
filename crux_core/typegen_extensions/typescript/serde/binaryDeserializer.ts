
/**
 * Copyright (c) Facebook, Inc. and its affiliates
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

import { Deserializer } from "./deserializer";

export abstract class BinaryDeserializer implements Deserializer {
  private static readonly BIG_32: bigint = BigInt(32);
  private static readonly BIG_64: bigint = BigInt(64);
  private static readonly textDecoder = new TextDecoder();
  public buffer: ArrayBuffer;
  public offset: number;

  constructor(data: Uint8Array) {