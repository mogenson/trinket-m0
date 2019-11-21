#ifndef hal_h_INCLUDED
#define hal_h_INCLUDED

bool pin_is_high(uint8_t pin);
bool pin_is_low(uint8_t pin);
uint8_t pin_new(enum gpio_port port, uint8_t pin);
void pin_into_input(uint8_t pin);
void pin_into_output(uint8_t pin);
void pin_into_pull_down_input(uint8_t pin);
void pin_into_pull_up_input(uint8_t pin);
void pin_set_high(uint8_t pin);
void pin_set_low(uint8_t pin);

#endif // hal_h_INCLUDED

