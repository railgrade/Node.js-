package com.redbadger.catfacts.shared_types;


public final class HttpHeader {
    public final String name;
    public final String value;

    public HttpHeader(String name, String value) {
        java.util.Objects.requireNonNull(name, "name must not be null");
        java.util.Objects.requireNonNull(value, "value must not be null");
        this.name = name;
        this.value = value;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_str(name);
        serializer.serialize_str(value);
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static HttpHeader deserialize(com.novi.serde.Deserializer deserializer) throws com.