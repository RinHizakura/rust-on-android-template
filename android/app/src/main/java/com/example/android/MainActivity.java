package com.example.android;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("greetings");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        RustUtils g = new RustUtils();
        String r = g.echoString("Amita");
        ((TextView)findViewById(R.id.textString)).setText(r);

        String i = g.echoInt(39);
        ((TextView)findViewById(R.id.textInt)).setText(i);
    }
}