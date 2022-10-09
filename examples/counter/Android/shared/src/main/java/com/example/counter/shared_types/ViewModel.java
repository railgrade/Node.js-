package com.example.counter.shared_types;


public final class ViewModel {
    public final String text;
    public final Boolean confirmed;

    public ViewModel(String text, Boolean confirmed) {
        java.util.Objects.requireNonNull(text, "text must not be null");
        java.util.Objects.requireNonNull(confirmed, "confirmed must not be null");
        this.text = text;
        this.confirmed = confirmed;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_str(text);
        serializer.serialize_bool(confirmed);
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static ViewModel deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_depth();
        Builder builder = new Builder();
        builder.text = deserializer.deserialize_str();
        builder.confirmed = deserializer.deserialize_bool();
        deserializer.decrease_container_depth();
        return builder.build();
    }

    public static ViewModel 