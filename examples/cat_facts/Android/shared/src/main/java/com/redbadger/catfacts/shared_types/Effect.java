package com.redbadger.catfacts.shared_types;


public abstract class Effect {

    abstract public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError;

    public static Effect deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        int index = deserializer.deserialize_variant_index();
        switch (index) {
            case 0: return Http.load(deserializer);
            case 1: return KeyValue.load(deserializer);
            case 2: return Platform.load(deserializer);
            case 3: return Render.load(deserializer);
            case 4: return Time.load(deserializer);
            default: throw new com.novi.serde.DeserializationError("Unknown variant index for Effect: " + index);
        }
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static Effect bcsDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bcs.BcsDeserializer(input);
        Effect value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
             throw new com.novi.serde.DeserializationError("Some input bytes were not read");
        }
        return value;
    }

    public static final class Http extends Effect {
        public final HttpRequest value;

        public Http(HttpRequest value) {
            java.util.Objects.requireNonNull(value, "value must not be null");
            this.value = value;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(0);
            value.serialize(serializer);
            serializer.decrease_container_depth();
        }

        static Http load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builde