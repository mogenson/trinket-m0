#include <hal_gpio.h>

/* wrap static inline functions so bindgen can pick them up */

uint8_t pin_new(enum gpio_port port, uint8_t pin) { return GPIO(port, pin); }

void pin_into_output(uint8_t pin) {
  gpio_set_pin_pull_mode(pin, GPIO_PULL_OFF);
  gpio_set_pin_direction(pin, GPIO_DIRECTION_OUT);
}

void pin_into_input(uint8_t pin) {
  gpio_set_pin_pull_mode(pin, GPIO_PULL_OFF);
  gpio_set_pin_direction(pin, GPIO_DIRECTION_IN);
}

void pin_into_pull_down_input(uint8_t pin) {
  gpio_set_pin_pull_mode(pin, GPIO_PULL_DOWN);
  gpio_set_pin_direction(pin, GPIO_DIRECTION_IN);
}

void pin_into_pull_up_input(uint8_t pin) {
  gpio_set_pin_pull_mode(pin, GPIO_PULL_UP);
  gpio_set_pin_direction(pin, GPIO_DIRECTION_IN);
}

bool pin_is_high(uint8_t pin) { return gpio_get_pin_level(pin); }

bool pin_is_low(uint8_t pin) { return !gpio_get_pin_level(pin); }

void pin_set_high(uint8_t pin) { gpio_set_pin_level(pin, true); }

void pin_set_low(uint8_t pin) { gpio_set_pin_level(pin, false); }

void pin_toggle(uint8_t pin) { gpio_toggle_pin_level(pin); }
