package com.redbadger.catfacts.shared_types;


public final class CatFact {
    public final String fact;
    public final Integer length;

    public CatFact(String fact, Integer length) {
        java.util.Objects.requireNonNull(fact, "fact must not be null");
        java.util.Objects.requireNonNull(length, "length must not be null");
        this.fact = fact;
        this.length = length;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_str(fact);
        serializer.serialize_i32(length);
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static CatFact deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_depth();
        Builder builder = new Builder();
        builder.fact = deserializer.deserialize_str();
        builder.length = deserializer.deserialize_i32();
   