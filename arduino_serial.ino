int inByte = 0;         // incoming serial byte

void setup() {
  // start serial port at 9600 bps:
  Serial.begin(9600);
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB port only
  }

  establishContact();  // send a byte to establish contact until receiver responds
}

void loop() {
  // if we get a valid byte, read analog ins:
  if (Serial.available() > 0) {
//    // get incoming byte:
//    inByte = Serial.read();
//    Serial.write(inByte);
    Serial.println("made contact. ready");
    delay(1000);
  }
}

void establishContact() {
  while (Serial.available() <= 0) {
    Serial.println("available");
    delay(1000);
  }
}