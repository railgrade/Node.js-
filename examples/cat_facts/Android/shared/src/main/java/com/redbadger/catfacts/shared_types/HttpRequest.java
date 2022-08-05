package com.redbadger.catfacts.shared_types;


public final class HttpRequest {
    public final String method;
    public final String url;
    public final java.util.List<HttpHeader> headers;

    public HttpRequest(String method, String url, java.util.List<HttpHeader> headers) {
        java.util.Objects.requireNonNull(method, "method must not be null");
        java.util.Objects.requireNonNull(url, "url must not be null");
        java.util.Objects.requireNonNull(headers, "headers must not be null");
        this.method = method;
        this.url = url;
        this.headers = headers;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_str(method);
        serializer.serialize_str(url);
        TraitHelpers.serialize_vector_HttpHeader(headers, serializer);
        serializer.decrease_container_depth();
    }

    public byte[] bcsSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bcs.BcsSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static HttpRequest deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_de