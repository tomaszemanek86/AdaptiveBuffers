#pragma once
#include <stdint>
#include <cstring>
#include <expected>

namespace core {

    // Deserialization

    enum DeserializerError {
        NotEnoughData,
        NotDeserializedYet
    };

    class IReader {
    public:
        virtual ~IReader() = default;
        virtual std::expected<void, DeserializerError> read(uint16_t offset, void* dest, uint16_t size) = 0; 
    };

    struct Context {
        Context(uint32_t* previous_offset, uint32_t** next_back_reference) : _previous_offset(previous_offset), _end_offset(0), _next_back_reference(next_back_reference) {}
        void set_size(uint32_t offset) {
            _end_offset = *_previous_end + offset;
            if (_next_back_reference != nullptr) {
                *_next_back_reference = &_end_offset;
            }
        }
        uint32_t* _previous_end;
        uint32_t _end_offset;
        uint32_t** _next_back_reference;
    };

    // Serialization

    template <typename T>
    struct Buffer {
        bool _is_set;
        T _data;
    };

    template <typename TBuffer, uint16_t Size>
    struct SizedArrayBuffer {
        uint16_t _size = 0;
        std::array<TBuffer, Size> _buffer;
    };

    template <typename TBuffer, typename TSerializer, uint16_t MaxSize>
    class MaxSizedArraySerializer {
    public:
        MaxSizedArraySerializer(SizedArrayBuffer<TBuffer>* buffer) : _buffer(buffer) {}
        inline TSerializer push() {
            if (_buffer->_size >= Size) {
                throw "Buffer overflow";
            }
            return TSerializer(&_buffer._buffer[_buffer->_size++]);
        }
        inline TSerializer with_current() {
            return TSerializer(_buffer);
        }
        inline uint16_t serialize(void *buffer) {
            auto offset = 0
            for (auto i = 0; i < _buffer->_size; i++) {
                offset += _buffer->_buffer[i].serialize(static_cast<void*>(buffer + offset));
            }
            return offset;
        }
        inline uint16_t size() {
            return Size;
        }
    private:
        SizedArrayBuffer<TBuffer, MaxSize>* _buffer;
    };

    template <typename TData, uint16_t MaxSize, uint16_t ItemSize>
    class MaxSizedArrayNativeSerializer {
    public:
        MaxSizedArrayNativeSerializer(SizedArrayBuffer<TBuffer>* buffer) : _buffer(buffer) {}
        inline MaxSizedArrayNativeSerializer<TData, MaxSize, ItemSize> push(TData value) {
            if (_buffer->_size >= Size) {
                throw "Buffer overflow";
            }
            _buffer->_size++;
            std::memcpy(_buffer._buffer[_buffer->_size], &alue, ItemSize);
            return *this;
        }
        inline uint16_t serialize(void *buffer) {
            auto offset = 0
            for (auto i = 0; i < _buffer->_size; i++) {
                memcpy(buffer + offset, &_buffer->_buffer[i], ItemSize);
                offset += ItemSize;
            }
            return Size;
        }
        inline uint16_t size() {
            return _buffer->_size;
        }
    private:
        SizedArrayBuffer<TData, MaxSize>* _buffer;
    };

}
