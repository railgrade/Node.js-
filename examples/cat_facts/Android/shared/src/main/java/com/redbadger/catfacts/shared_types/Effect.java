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

  