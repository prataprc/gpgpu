 Ideas    |  Point             |  Vector              | Matrix                      | Transforms
----------|--------------------|----------------------|-----------------------------|------------------------
points    |   position         |  direction           |   determinant               | identity transform
vector    |   homogenous-point |  magnitude |A|       |   basis (Change of basis)   | translate transform
matrix    |                    |  unit-vector Â       |   minor of matrix           | scale transform
normal    |                    |  dot-product A.B     |   cofactor of matrix        | rotate transform
tangent   |                    |  cross-product AxB   |   adjoint of matrix         | perpective transform
bitangent |                    |  vector-angle θ      |   inverse of matrix         | shear transform
surface   |                    |                      |   orthogonal matrix         | affine transform
          |                    |                      |   perspective matrix        |

* vector-angle = invcos(unit-A.unit-B) that is, dot product of unit-vectors

Spherical coordinate system (r, teta, phi), teta is polar-angle, phi is azhimuth angle.

Tx  Ty  Tz  0       tangent    (right)
Bx  By  Bz  0       bi-tangent (forward)
Nx  Ny  Nz  0       normal
0   0   0   1

sin             opposite
cosine          adjasent
tangent         hypotenuse
arcsin
arccosine
arctangent
secant
cosecant
cotan


* natural numbers - whole numbers - rational numbers - irrational numbers.
* complex numbers.

* algebraic expressions - add, sub, mult, div and exp - exp of rational numbers.
* polynomial - (linear, 1) - (quadratic, 2) - (cubic, 3), (quartic, 4), (quintic, 5).
* polynomial - made of terms, combined using add-op and sub-op - uniterm, biterm, triterm.
* univariate - expression with one variable; bivariate, trivariate.
* binomial-theorem, binomial-form, pascal-triangle.
* parametric-functions

* linear-interpolation, bi-linear, tri-linear, bezier-surfaces.

* permutation, order matter => nPr => n! / (n-r)!
* combination, order does not matter => nPr => n! / (r! * (n-r)!)

#### cgmath

conv    Constrained conversion functions for assisting in situations where type inference is difficult.


`Point1`, `Point2`, `Point3`, `Vector1`, `Vector2`, `Vector3`, `Vector4`, `Matrix2`, `Matrix3`, `Matrix4`

`AbsDiff`           The requisite parameters for testing for approximate equality using a absolute difference based comparison.
`Basis2`            A two-dimensional rotation matrix.
`Basis3`            A three-dimensional rotation matrix.
`Decomposed`        A generic transformation consisting of a rotation, displacement vector and scale amount.
`Deg`               An angle, in degrees.
`Euler`             A set of Euler angles representing a rotation in three-dimensional space.
`Ortho`             An orthographic projection with arbitrary left/right/bottom/top distances
`Perspective`       A perspective projection with arbitrary left/right/bottom/top distances
`PerspectiveFov`    A perspective projection based on a vertical field-of-view angle.
`Quaternion`        A quaternion in scalar/vector form.
`Rad`               An angle, in radians.
`Relative`          The requisite parameters for testing for approximate equality using a relative based comparison.
`Ulps`              The requisite parameters for testing for approximate equality using an ULPs based comparison.

`AbsDiffEq`         Equality that is defined using the absolute difference of two numbers.
`Angle`             Angles and their associated trigonometric functions.
`Array`             An array containing elements of type Element
`BaseFloat`         Base floating point types
`BaseNum`           Base numeric types with partial ordering
`Bounded`           Numbers which have upper and lower bounds
`ElementWise`       Element-wise arithmetic operations. These are supplied for pragmatic reasons, but will usually fall outside of traditional algebraic properties.
`EuclideanSpace`    Points in a Euclidean space with an associated space of displacement vectors.
`InnerSpace`        Vectors that also have a dot (or inner) product.
`Matrix`            A column-major matrix of arbitrary dimensions.
`MetricSpace`       A type with a distance function between values.
`One`               Defines a multiplicative identity element for Self.
`RelativeEq`        Equality comparisons between two numbers using both the absolute difference and relative based comparisons.
`Rotation`          A trait for a generic rotation. A rotation is a transformation that creates a circular motion, and preserves at least one point in the space.
`Rotation2`         A two-dimensional rotation.
`Rotation3`         A three-dimensional rotation.
`SquareMatrix`      A column-major major matrix where the rows and column vectors are of the same dimensions.
`Transform`         A trait representing an affine transformation that can be applied to points or vectors. An affine transformation is one which
`Transform2`
`Transform3`
`UlpsEq`            Equality comparisons between two numbers using both the absolute difference and ULPs (Units in Last Place) based comparisons.
`VectorSpace`       Vectors that can be added together and multiplied by scalars.
`Zero`              Defines an additive identity element for Self.

`dot`           Dot product of two vectors.
`frustum`       Create a perspective matrix from a view frustum.
`ortho`         Create an orthographic projection matrix.
`perspective`   Create a perspective projection matrix.
`point1`        The short constructor.
`point2`        The short constructor.
`point3`        The short constructor.
`vec1`          The short constructor.
`vec2`          The short constructor.
`vec3`          The short constructor.
`vec4`          The short constructor.

`abs_diff_eq`           Approximate equality of using the absolute difference.
`abs_diff_ne`           Approximate inequality of using the absolute difference.
`assert_abs_diff_eq`    An assertion that delegates to abs_diff_eq!, and panics with a helpful error on failure.
`assert_abs_diff_ne`    An assertion that delegates to abs_diff_ne!, and panics with a helpful error on failure.
`assert_relative_eq`    An assertion that delegates to relative_eq!, and panics with a helpful error on failure.
`assert_relative_ne`    An assertion that delegates to relative_ne!, and panics with a helpful error on failure.
`assert_ulps_eq`        An assertion that delegates to ulps_eq!, and panics with a helpful error on failure.
`assert_ulps_ne`        An assertion that delegates to ulps_ne!, and panics with a helpful error on failure.
`relative_eq`           Approximate equality using both the absolute difference and relative based comparisons.
`relative_ne`           Approximate inequality using both the absolute difference and relative based comparisons.
`ulps_eq`               Approximate equality using both the absolute difference and ULPs (Units in Last Place).
`ulps_ne`               Approximate inequality using both the absolute difference and ULPs (Units in Last Place).
