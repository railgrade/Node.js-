package com.redbadger.catfacts.shared_types;


public final class RenderOperation {
    public RenderOperation() {
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static RenderOperation deserialize(com.novi.serde.Deserializer deserializer) throws com.n