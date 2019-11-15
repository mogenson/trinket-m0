#include <hal_gpio.h>

/* wrap static inline functions so bindgen can pick them up */

uint8_t pin_new(enum gpio_port port, uint8_t pin) {
    return GPIO(port, pin);
}

void pin_set_high(uint8_t pin) {
    gpio_set_pin_level(pin, true);
}

void pin_set_low(uint8_t pin) {
    gpio_set_pin_level(pin, false);
}

void hal_gpio_set_pin_direction(const uint8_t pin, const enum gpio_direction direction) {
    gpio_set_pin_direction(pin, direction);
}
