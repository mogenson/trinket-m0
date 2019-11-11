#ifndef hal_h_INCLUDED
#define hal_h_INCLUDED

void hal_gpio_set_pin_direction(const uint8_t pin, const enum gpio_direction direction);
void hal_gpio_toggle_pin_level(const uint8_t pin);

#endif // hal_h_INCLUDED

