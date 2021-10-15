package com.example.android;

/**
 * Created by emilytoop on 01/09/2017.
 */

public class RustUtils {

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}