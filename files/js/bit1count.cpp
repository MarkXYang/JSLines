#include <iostream>
#include <cstdint>

std::pair<int, int> countMaxContinuousBit1(uint32_t num);

std::pair<int, int> countMaxContinuousBit1(uint32_t num) {
    int count = 0;
    int times_shift = 0;
    int maxCount = 0;
    int startBit = -1;
    int maxStartBit = -1;


    while (num > 0) {
        if (num & 1) {
            count++;
            if (startBit == -1) {
                startBit = 31 - __builtin_clz(num);
            }
            if (count > maxCount) {
                maxCount = count;
                maxStartBit = times_shift;
            }
        } else {
            count = 0;
            startBit = -1;
        }
        num >>= 1;
        ++times_shift;
    }

    return std::make_pair(maxCount, maxStartBit);
}

int main()
{
    uint32_t x = 0b0011'1111'0001'1111'1110'1111'1111'1100;
    auto result = countMaxContinuousBit1(x);
    std::cout << "Max continuous bit 1 count: " << result.first << std::endl;
    std::cout << "Starting bit position: " << result.second << std::endl;

    uint32_t y = 0b0011'1011'0001'1110'1110'1110'1110'1111;
    result = countMaxContinuousBit1(y);
    std::cout << "Max continuous bit 1 count: " << result.first << std::endl;
    std::cout << "Starting bit position: " << result.second << std::endl;

    y = 0b0000'0110'0000'0000'0000'0000'0000'0000;
    result = countMaxContinuousBit1(y);
    std::cout << "Max continuous bit 1 count: " << result.first << std::endl;
    std::cout << "Starting bit position: " << result.second << std::endl;

    y = 0b0000'0000'0000'0000'0000'0000'0000'0000;
    result = countMaxContinuousBit1(y);
    std::cout << "Max continuous bit 1 count: " << result.first << std::endl;
    std::cout << "Starting bit position: " << result.second << std::endl;
    return 0;
}