
package com.example.counter.shared_types;


public abstract class SseResponse {

    abstract public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError;

    public static SseResponse deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {