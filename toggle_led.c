#include <hal_gpio.h>

void toggle_led() {
    gpio_toggle_pin_level(10);
}
