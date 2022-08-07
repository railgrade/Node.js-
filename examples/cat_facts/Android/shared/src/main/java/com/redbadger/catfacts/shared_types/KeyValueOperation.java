package com.redbadger.catfacts.shared_types;


public abstract class KeyValueOperation {

    abstract public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError;

    public static KeyValueOperation deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        int index = deserializer.deserialize_variant_index();
        switch (index) {
            case 0: return Read.load(deserializer);
            case 1: return Write.load(deserializer);
            default: throw new com.novi.serde.DeserializationError("Unknown variant index for KeyValueOperation: " + index);
        }
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static KeyValueOperation bcsDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
            