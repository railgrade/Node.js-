public final class Requests {

  public static java.util.List<Request> bcsDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
    if (input == null) {
      throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
    }
    com.novi.serde.Deserializer deserializer = new com.novi.bcs.BcsDeserializer(input);
    deserializer.increase_container_depth();

    long length = deserializer.deserialize_len();

    java.util.List<Request> value = new java.util.ArrayList<>();

    for 