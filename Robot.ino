int pinB1 = 16; // D0 -> IN1//ซ้าย
int pinB2 = 5; // D1 -> IN2
int pinA1 = 4; // D2 -> IN3//ขวา
int pinA2 = 0;  //  D3 -> IN4
void setup() {
  pinMode(pinA1, OUTPUT);
  pinMode(pinA2, OUTPUT);
  pinMode(pinB1, OUTPUT);
  pinMode(pinB2, OUTPUT);
}
void forward()
{
  digitalWrite(pinA1, HIGH); //ขวา
  digitalWrite(pinA2, LOW);
  digitalWrite(pinB1, HIGH);//ซ้าย
  digitalWrite(pinB2, LOW);
}
void backward()
{
  digitalWrite(pinA1, LOW);
  digitalWrite(pinA2, HIGH);
  digitalWrite(pinB1, LOW);
  digitalWrite(pinB2, HIGH);
}
void LEFT()
{
  digitalWrite(pinA1, HIGH); //ขวาเดินหน้า
  digitalWrite(pinA2, LOW);
  digitalWrite(pinB1, LOW); //ซ้ายถอย
  digitalWrite(pinB2, HIGH);
}
void RIGHT()
{
  digitalWrite(pinA1, LOW); //ขวาถอย
  digitalWrite(pinA2, HIGH);
  digitalWrite(pinB1, HIGH);//ซ้ายหน้า
 digitalWrite(pinB2, LOW);
}
void stopBot()
{
  digitalWrite(pinA1, LOW);
  digitalWrite(pinA2, LOW);
  digitalWrite(pinB1, LOW);
  digitalWrite(pinB2, LOW);
}
void loop () {
   stopBot ();
  delay (1000);
  forward ();
  delay (2000);
  stopBot ();
  delay (1000);
  backward();
  delay (2000);
  stopBot ();
  delay (1000);
  LEFT();
  delay (2000);
  stopBot ();
  delay (1000);
  RIGHT ();
  delay (2000);
}