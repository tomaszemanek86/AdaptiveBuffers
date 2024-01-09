#pragma once
#include <cstdint>
#include <concepts>
#include <variant>
#include <cstring>

namespace abf {

enum ErrorCode {
    ReadFailed,
    WriteFailed,
    NotSet,
    AlreadySet,
    Overflow
};

struct Error {
    ErrorCode error_code;
    uint16_t object_id;
};

template <typename TValue>
struct Result {

    Result(TValue value)
        : value_(value), ok_(true) {
    }

    Result(Error error)
        : error_(error), ok_(false) {
    }

    ~Result() {
        if (ok_) {
            value_.~TValue();
        } else {
            error_.~TErr();
        }
    }

    bool is_ok() const {
        return ok_;
    }

    bool is_err() const {
        return !ok_;
    }

    template <typename = std::enable_if<!std::is_same_v<TValue, void>>>
    const TValue& value() const {
        if (!ok_) {
            std::cerr << "not ok" << std::endl;
            exit(1);
        }
        return value_;
    }

    template <typename = std::enable_if<!std::is_same_v<TValue, void>>>
    TValue& value() {
        if (!ok_) {
            std::cerr << "not ok" << std::endl;
            exit(1);
        }
        return value_;
    }

    const TErr& error() const {
        if (ok_) {
            std::cerr << "not error" << std::endl;
            exit(1);
        }
        return error_;
    }


    union {
        TValue value_;
        Error error_;
    };
    bool ok_;
};

////////////////////////////////////////////////////////////////////////////////
// SERIALIZER
////////////////////////////////////////////////////////////////////////////////

template <typename TWriter>
struct Writer {

    Writer(TWriter writer) : writer_(writer) {}
    
    // writes data and returns new writer that points to following data
    Result<TWriter> write(void *source, uin32_t size) {
        return writer_.write(source, size);
    }

    // shifts write index and returns new writer that points to following data
    Result<TWriter> skip(int32_t offset) {
        return writer_.skip(offset);
    }

    // return current writer
    TWriter current() {
        return writer_.current();
    }

    // return current writer
    uint32_t distance(TWriter previous) {
        return writer_.distance(previous);
    }


    TWriter writer_;
};

template <typename TData>
struct Buffer {
    Buffer() : valid(false), data() {}

    bool valid;
    TData data;
};

template <typename TSerializer>
struct Serializer {

    Serializer(TSerializer inst) : inst_(inst) {}

    // serializes data and returns writer pointing behind serialized data
    template <typename TWriter>
    Result<Writer<TWriter>> write(Writer<TWriter> writer) {
        return inst_.serialize(writer);
    }

    template <typename TWriter>
    Result<Writer<TWriter>> skip(Writer<TWriter> writer) {
        return inst_.skip(writer);
    }


    Buffer<TSerializer> inst_;
};

template <typename TData, uint32_t Size>
struct SizedDataSerializer {

    SizedDataSerializer() {}

    template <typename TWriter>
    Result<void, Error> serialize(Writer<TWriter> writer) {
        return writer.write(&data_, Size);
    }

    void set_data(TData data) {
        data_ = data;
    }

    TData data_;
};

struct SizeSerializer {

    SizeSerializer(uint8_t bytes) : bytes_(bytes) {}

    template <typename TWriter>
    Result<void, Error> serialize(Writer<TWriter> writer) {
        return writer.write();
    }

    Result<void> set_size(uint32_t size) {
        if (bytes_ == 1 && size > 255) {
            return Result<void>(ErrorCode::Overflow);
        }
        if (bytes_ == 2 && size > 65000) {
            return Result<void>(ErrorCode::Overflow);
        }
        if (bytes_ == 1 && size > 4000000000) {
            return Result<void>(ErrorCode::Overflow);
        }
        size_ = size;
        return Result<void>();
    }


    uint8_t bytes_;
    uint32_t size_;
}

template <typename TThisSerializer>
struct SizeSerializer {

    SizeSerializer(
        TThisSerializer this_serializer,
        SizeSerializer& size_serializer
    )   : this_serializer_(this_serializer)
        , size_serializer_(size_serializer) {}

    template <typename TWriter>
    Result<void, Error> serialize(Writer<TWriter> writer) {
        auto next = this_serializer_.serialize(writer);
        if (result.is_error()) {
            return Result<void, Error>(result.error());
        }
        size_serializer_.set_size(writer.distance(next));
        return size_serializer_.serialize();
    }


    TThisSerializer this_serializer_;
    TSizeSerializer& size_serializer_;
};

template <typename TSerializer>
struct GenericSerializer {

    GenericSerializer(const TSerializer* serializer) : serializer_(serializer) {}

    template <typename TWriter>
    Result<void, Error> serialize(Writer<TWriter> writer) {
        return serializer_.serialize(writer);
    }


    const TSerializer* serializer_;
};



////////////////////////////////////////////////////////////////////////////////
// DESERIALIZER
////////////////////////////////////////////////////////////////////////////////

template <typename TReader>
struct Reader {

    Reader(TReader reader) : reader_(reader) {}

    // reads data and returns new reader that points to following data
    Result<Reader<TReader>> read(void *source, uin32_t size) {
        return reader_.read(source, size);
    }


    TReader reader_;
};

}
