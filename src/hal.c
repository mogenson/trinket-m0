#include <hal_gpio.h>

/* wrap static inline functions so bindgen can pick them up */

void hal_gpio_set_pin_direction(const uint8_t pin, const enum gpio_direction direction) {
    gpio_set_pin_direction(pin, direction);
}

void hal_gpio_toggle_pin_level(const uint8_t pin) {
    gpio_toggle_pin_level(pin);
}

void hal_gpio_set_pin_level(const uint8_t pin, const bool level) {
    gpio_set_pin_level(pin, level);
}
