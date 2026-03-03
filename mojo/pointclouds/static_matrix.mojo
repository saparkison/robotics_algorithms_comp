from memory import UnsafePointer

struct OwnData[dtype: DType]:  # TODO: implement `Bufferable` trait
    var ptr: UnsafePointer[Scalar[dtype]]

    fn __init__(out self, size: Int):
        """
        Allocate given space on memory.
        The bytes allocated is `size` * `byte size of dtype`.

        Notes:
        `ndarray.flags['OWN_DATA']` should be set as True.
        The memory should be freed by `__del__`.
        """
        self.ptr = UnsafePointer[Scalar[dtype]]().alloc(size)

    fn __init__(out self, ptr: UnsafePointer[Scalar[dtype]]):
        """
        Do not use this if you know what it means.
        If the pointer is associated with another array, it might cause
        dangling pointer problem.

        Notes:
        `ndarray.flags['OWN_DATA']` should be set as False.
        The memory should not be freed by `__del__`.
        """
        self.ptr = ptr

    fn __moveinit__(out self, owned other: Self):
        self.ptr = other.ptr

    fn get_ptr(self) -> UnsafePointer[Scalar[dtype]]:
        return self.ptr


struct StaticMatrix[rows: UInt, cols: UInt, dtype: DType = DType.float32]()

    alias width: Int = simdthof[dtype]()

    alias shape: Tuple[Uint, Uint] = [rows, cols]

    alias size: UInt = rows * cols

    var _buf: OwnData[Scalar[dtype]]

    @always_inline
    fn __init__ (out self):
        self._buf = OwnData[dtype](size=self.size)


fn matmul[
    dtype: DType
](A: StaticMatrix[dtype], B: StaticMatrix[dtype]) raises -> StaticMatrixMatrix[dtype]:
    """
    Matrix multiplication.

    ```
    """

    alias width = max(simdwidthof[dtype](), 16)

    if A.shape[1] != B.shape[0]:
        raise Error(
            String("Cannot matmul {}x{} matrix with {}x{} matrix.").format(
                A.shape[0], A.shape[1], B.shape[0], B.shape[1]
            )
        )

    var C: Matrix[dtype]

    C = StaticMatrix.zeros[dtype](shape=(A.shape[0], B.shape[1]), order=B.order())

    @parameter
    fn calculate_CC(m: Int):
        for k in range(A.shape[1]):

            @parameter
            fn dot[simd_width: Int](n: Int):
                C._store[simd_width](
                    m,
                    n,
                    C._load[simd_width](m, n)
                    + A._load(m, k) * B._load[simd_width](k, n),
                )

            vectorize[dot, width](B.shape[1])

    parallelize[calculate_CC](A.shape[0], A.shape[0])
    
    var _A = A
    var _B = B

    return C^