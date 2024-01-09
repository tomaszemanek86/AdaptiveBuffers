pub static SOURCE: &str = "
#pragma once
#include <cstdint>
#include <cstring>
#include <stdexcept>
#include <vector>
#include <limits>

#include <iostream>
#define LOGE(msg) std::cerr << msg << std::endl;

namespace abf {
<<BSWAP_SOURCE>>
    ////////////////////////////////////////////////////////////////////////////////
    // SERIALIZER
    ////////////////////////////////////////////////////////////////////////////////

    template <typename TData, uint32_t Size>
    class NativeSerializer {
    public:
        using Data = TData;

        NativeSerializer() : data_(), set_(false) {}

        uint32_t serialize(uint8_t* dest) {
            if (!set_) {
                throw std::runtime_error(\"Not set\");
            }
            copy(dest, &data_, Size);
            return Size;
        }

        void set_data(TData data) {
            data_ = data;
            set_ = true;
        }

        uint32_t size() {
            return Size;
        }

        void init() {
            set_ = false;
        }

    private:
        TData data_;
        bool set_;
    };

    template <typename TSerializer>
    class LazySerializer {
    public:
        using Data = TSerializer::Data;

        LazySerializer() : serializer_(), set_(false) {}

        uint32_t serialize(uint8_t* dest) {
            dest_ = dest;
            return serializer_.size();
        }

        void set_data(Data data) {
            serializer_.set_data(data);
            serializer_.serialize(dest_);
        }

        uint32_t size() {
            return serializer_.size();
        }

        void init() {
            serializer_.init();
        }

    private:
        TSerializer serializer_;
        uint8_t* dest_;
        bool set_;
    };

    class IViewKeySetter {
    public:
        ~IViewKeySetter() = default;

        virtual void set_u8(uint8_t data) = 0;
        virtual void set_u16(uint16_t data) = 0;
        virtual void set_u32(uint32_t data) = 0;
        virtual void set_u64(uint64_t data) = 0;
    };

    template <typename TNativeData, uint32_t NativeSize>
    class ViewKeySerializer : public IViewKeySetter {
        friend class ViewKeySetter;
    public:
        ViewKeySerializer() : native_() {}

        uint32_t serialize(uint8_t* dest) {
            return native_.serialize(dest);
        }

        void set_u8(uint8_t data) override {
            native_.set_data(static_cast<TNativeData>(data));
        }

        void set_u16(uint16_t data) override {
            native_.set_data(static_cast<TNativeData>(data));
        }

        void set_u32(uint32_t data) override {
            native_.set_data(static_cast<TNativeData>(data));
        }

        void set_u64(uint64_t data) override {
            native_.set_data(static_cast<TNativeData>(data));
        }

        uint32_t size() {
            return native_.size();
        }

        void init() {
            native_.init();
        }

    private:
        LazySerializer<NativeSerializer<TNativeData, NativeSize>> native_;
    };

    template <typename TSerialzer, uint32_t Size>
    class ArraySerializer {
    public:
        using ItemSerializer = TSerialzer;

        ArraySerializer() : serializers_() {}

        uint32_t serialize(uint8_t* dest) {
            uint8_t* current_dest = dest;
            for (auto& si : serializers_) {
                current_dest += si.serialize(current_dest);
            }
            return current_dest - dest;
        }

        TSerialzer& get(uint32_t i) {
            if (i > Size) {
                throw std::runtime_error(\"Out of range\");
            }
            return serializers_[i];
        }

        void init() {
            for (auto i = 0; i < Size; i++) {
                serializers_[i].init();
            }
        }

        uint32_t size() {
            uint32_t all_sizes = 0;
            for (auto& si : serializers_) {
                all_sizes += si.size();
            }
            return all_sizes;
        }

        uint32_t length() {
            return Size;
        }

    private:
        TSerialzer serializers_[Size];
    };

    template <typename TSerialzer>
    class DynArraySerializer {
    public:
        using ItemSerializer = TSerialzer;

        DynArraySerializer() : serializers_() {}

        uint32_t serialize(uint8_t* dest) {
            uint8_t* current_dest = dest;
            for (auto& si : serializers_) {
                current_dest += si.serialize(current_dest);
            }
            return current_dest - dest;
        }

        TSerialzer& get(uint32_t i) {
            while (i + 1 > serializers_.size()) {
                serializers_.push_back(TSerialzer());
                serializers_.back().init();
            }
            return serializers_[i];
        }

        void init() {
            for (auto& si : serializers_) {
                si.init();
            }
        }

        uint32_t size() {
            uint32_t all_sizes = 0;
            for (auto& si : serializers_) {
                all_sizes += si.size();
            }
            return all_sizes;
        }

        uint32_t length() {
            return serializers_.size();
        }

    private:
        std::vector<TSerialzer> serializers_;
    };

    template <typename TArraySerialzer, typename TSizeSerializer>
    class ArraySizedSerializer {
    public:
        ArraySizedSerializer() : array_(), size_() {}

        uint32_t serialize(uint8_t* dest) {
            size_->set_data(array_.length());
            return array_.serialize(dest);
        }

        TArraySerialzer::ItemSerializer& get(uint32_t i) {
            return array_.get(i);
        }

        void init() {
            array_.init();
            size_->init();
        }

        void set_size_serializer(TSizeSerializer *size) {
            size_ = size;
        }

    private:
        TArraySerialzer array_;
        TSizeSerializer *size_;
    };

    template <typename TSerialzer, typename TDataFrom>
    class IntCastSerializer {
    public:
        IntCastSerializer() : serializer_() {}

        void set_data(TDataFrom data) {
            using DataType = typename TSerialzer::Data;

            if (data < std::numeric_limits<DataType>::min()) {
                throw std::runtime_error(\"minimum violated\");
            }
            if (data > std::numeric_limits<DataType>::max()) {
                throw std::runtime_error(\"maximum violated\");
            }
            serializer_.set_data(reinterpret_cast<DataType>(data));
        }

        uint32_t serialize(uint8_t* dest) {
            return serializer_.serialize(dest);
        }

        void init() {
            serializer_.init();
        }

    private:
        TSerialzer serializer_;
    };

    ////////////////////////////////////////////////////////////////////////////////
    // DESERIALIZER
    ////////////////////////////////////////////////////////////////////////////////

    template <typename TData, uint32_t Size>
    class NativeDeserializer {
    public:
        NativeDeserializer() : source_(nullptr) {}
        NativeDeserializer(uint8_t* source) : source_(source) {}

        TData get_data() {
            if (!_deserialized()) {
                throw std::runtime_error(\"Source not set\");
            }
            TData value;
            copy(&value, source_, Size);
            return value;
        }

        void _set_source(uint8_t *source) {
            source_ = source;
        }

        bool _source_set() {
            return source_ != nullptr;
        }

        bool _deserialized() {
            return source_ != nullptr;
        }

        uint8_t* _end() {
            return source_ + Size;
        }

        void init() {
            source_ = nullptr;
        }

    private:
        uint8_t *source_;
    };

    template <typename TDeserialzer, uint32_t Size>
    class ArrayDeserializer {
    public:
        using ItemDeserializer = TDeserialzer;

        ArrayDeserializer() {
            init();
        }

        ArrayDeserializer(uint8_t* source) {
            init();
            _set_source(source);
        }

        TDeserialzer& get(uint32_t i) {
            if (!deserializers_[i]._source_set()) {
                if (i > 0) {
                    if (deserializers_[i - 1]._deserialized()) {
                        deserializers_[i]._set_source(deserializers_[i - 1]._end());
                    } else {
                        throw std::runtime_error(\"Previous member not deserialized\");
                    }
                } else {
                    throw std::runtime_error(\"First member not deserialized\");
                }
            }
            return deserializers_[i];
        }

        void init() {
            for (auto i = 0; i < Size; i++) {
                deserializers_[i].init();
            }
        }

        void _set_source(uint8_t *source) {
            deserializers_[0]._set_source(source);
        }

        bool _source_set() {
            return deserializers_[0]._source_set();
        }

        bool _deserialized() {
            return deserializers_[Size - 1]._deserialized();
        }

        uint8_t* _end() {
            return deserializers_[Size - 1]._end();
        }

    private:
        TDeserialzer deserializers_[Size];
    };

    template <typename TDeserialzer>
    class DynArrayDeserializer {
    public:
        using ItemDeserializer = TDeserialzer;

        DynArrayDeserializer() {
            init();
        }

        DynArrayDeserializer(uint8_t* source) {
            init();
            _set_source(source);
        }

        TDeserialzer& get(uint32_t i) {
            while (deserializers_.size() < i + 1) {
                deserializers_.push_back(TDeserialzer());
                deserializers_.back().init();
            }
            if (!deserializers_[i]._source_set()) {
                if (i == 0) {
                    if (!_source_set()) {
                        throw std::runtime_error(\"Source not set\");
                    }
                    deserializers_[0]._set_source(source_);
                }
                if (i > 0) {
                    if (deserializers_[i - 1]._deserialized()) {
                        deserializers_[i]._set_source(deserializers_[i - 1]._end());
                    } else {
                        throw std::runtime_error(\"Previous member not deserialized\");
                    }
                }
            }
            return deserializers_[i];
        }

        void init() {
            deserializers_ = std::vector<TDeserialzer>();
        }

        void _set_source(uint8_t *source) {
            source_ = source;
        }

        bool _source_set() {
            return source_ != nullptr;
        }

        bool _deserialized() {
            if (deserializers_.size()) {
                return deserializers_.back()._deserialized();
            } else {
                return _source_set();
            }
        }

        uint8_t* _end() {
            if (deserializers_.size()) {
                return deserializers_.back()._end();
            } else {
                return source_;
            }
        }

    private:
        std::vector<TDeserialzer> deserializers_;
        uint8_t* source_;
    };

    template <typename TIArrayDeserialzer, typename TSizeDeserialzer>
    class ArraySizedDeserializer {
    public:
        using ItemDeserializer = TIArrayDeserialzer::ItemDeserializer;

        ArraySizedDeserializer() {
            init();
        }

        ArraySizedDeserializer(uint8_t* source) {
            init();
            _set_source(source);
        }

        ItemDeserializer& get(uint32_t i) {
            if (!size_->_deserialized()) {
                throw std::runtime_error(\"Array size not deserialized yet\");
            }
            if (i >= size_->get_data()) {
                throw std::runtime_error(\"Array size not deserialized yet\");
            }
            return array_.get(i);
        }

        void init() {
            array_ .init();
        }

        void _set_source(uint8_t *source) {
            array_._set_source(source);
        }

        bool _source_set() {
            return array_._source_set();
        }

        bool _deserialized() {
            return array_._deserialized();
        }

        uint8_t* _end() {
            return array_._end();
        }

        void set_size_deserializer(TSizeDeserialzer *size) {
            size_ = size;
        }

    private:
        TIArrayDeserialzer array_;
        TSizeDeserialzer* size_;
    };

}
";

pub static BSWAP_SOURCE: &str = "
inline uint8_t bswap8(uint8_t value) {
    value = ((value & 0x55) << 1) | ((value & 0xAA) >> 1);
    value = ((value & 0x33) << 2) | ((value & 0xCC) >> 2);
    value = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
    return value;
}

inline uint16_t bswap16(uint16_t value) {
    return (static_cast<uint16_t>(bswap8(static_cast<uint8_t>(value))) << 8) |
           (bswap8(static_cast<uint8_t>(value >> 8)));
}

inline uint32_t bswap32(uint32_t value) {
    return (static_cast<uint32_t>(bswap16(static_cast<uint16_t>(value))) << 16) |
           (bswap16(static_cast<uint16_t>(value >> 16)));
}

inline uint64_t bswap64(uint64_t value) {
    return (static_cast<uint64_t>(bswap32(static_cast<uint32_t>(value))) << 32) |
           (bswap32(static_cast<uint32_t>(value >> 32)));
}
inline void bswap8_ptr(uint8_t* ptr) {
    *ptr = bswap8(*ptr);
}
inline void bswap16_ptr(uint16_t* ptr) {
    *ptr = bswap16(*ptr);
}
inline void bswap32_ptr(uint32_t* ptr) {
    *ptr = bswap32(*ptr);
}
inline void bswap64_ptr(uint64_t* ptr) {
    *ptr = bswap64(*ptr);
}

inline void copy(void* dest, void* source, size_t size) {
    std::memcpy(dest, source, size);
    switch (size) {
        case 1:
            bswap8_ptr(static_cast<uint8_t*>(dest));
            break;
        case 2:
            bswap16_ptr(static_cast<uint16_t*>(dest));
            break;
        case 4:
            bswap32_ptr(static_cast<uint32_t*>(dest));
            break;
        case 8:
            bswap64_ptr(static_cast<uint64_t*>(dest));
            break;
    }
}
";

pub static NO_BSWAP_SOURCE: &str = "
inline void copy(void* dest, void* source, size_t size) {
    std::memcpy(dest, source, size);
}
";
