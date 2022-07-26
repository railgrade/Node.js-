package com.redbadger.catfacts.shared_types;


public final class CatImage {
    public final String file;

    public CatImage(String file) {
        java.util.Objects.requireNonNull(file, "file must not be null");
        this.file = file;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_str(file);
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static CatImage deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_depth();
        Builder builder = new Builder();
        builder.file = deserializer.deserialize_str();
        deserializer.decrease_container_depth();
        return builder.build();
    }

    public static CatImage bcsDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bcs.BcsDeserializer(input);
        CatImage value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
             throw new com.