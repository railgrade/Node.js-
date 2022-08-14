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
