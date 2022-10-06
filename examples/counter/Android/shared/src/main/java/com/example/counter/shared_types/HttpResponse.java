package com.example.counter.shared_types;


public final class HttpResponse {
    public final @com.novi.serde.Unsigned Short status;
    public final java.util.List<@com.novi.serde.Unsigned Byte> body;

    public HttpResponse(@com.novi.serde.Unsigned Short status, java.util.List<@com.novi.serde.Unsigned Byte> body) {
        java.util.Objects.requireNonNull(status, "status must not be null");
        java.util.Objects.requireNonNull(body, "body must not be null");
        this.status = status;
        this.body = body;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_u16(status);
        TraitHelpers.serialize_vector_u8(body, serializer);
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static HttpResponse deseriali