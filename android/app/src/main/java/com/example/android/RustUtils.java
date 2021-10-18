package com.example.android;

public class RustUtils {

    private static native String echostring(final String pattern);
    private static native String echoint(final int pattern);

    public String echoString(String to) {
        return echostring(to);
    }

    public String echoInt(int to) {
        return echoint(to);
    }
}