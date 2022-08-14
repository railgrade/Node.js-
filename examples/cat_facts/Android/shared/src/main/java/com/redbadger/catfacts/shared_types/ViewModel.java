package com.redbadger.catfacts.shared_types;


public final class ViewModel {
    public final String fact;
    public final java.util.Optional<CatImage> image;
    public final String platform;

    public ViewModel(String fact, java.util.Optional<CatImage> image, String platform) {
        java.util.Objects.requireNonNull(fact, "fact must not be null");
        java.util.Objects.requireNonNull(image, "image must not be null");
        java.util.Objects.requireNonNull(platform, "platform must not be null");
        this.fact = fact;
        this.image = image;
        this.platform = platform;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_str(fact);
        TraitHelpers.serialize_option_CatImage(image, serializer);
        serializer.serialize_str(platform);
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
        builder.fact = deserializer.deserialize_str();
        builder.image = TraitHelpers.deserialize_option_CatImage(deserializer);
        builder.platform = deserializer.deserialize_str();
        deserializer.decrease_container_depth();
        return builder.build();
    }

    public static ViewModel bcsDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bcs.BcsDeserializer(input);
        ViewModel value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
 