#ifndef hal_h_INCLUDED
#define hal_h_INCLUDED

void hal_gpio_set_pin_direction(const uint8_t pin, const enum gpio_direction direction);

uint8_t pin_new(enum gpio_port port, uint8_t pin);
void pin_set_high(uint8_t pin);
void pin_set_low(uint8_t pin);

#endif // hal_h_INCLUDED

