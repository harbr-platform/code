<template>
  <div>
    <svg
      class="rotate outer"
      width="100%"
      height="100%"
      viewBox="0 0 590 591"
      xmlns="http://www.w3.org/2000/svg"
      xml:space="preserve"
      style="fill-rule: evenodd; clip-rule: evenodd; stroke-linejoin: round; stroke-miterlimit: 2"
    >
      <g transform="matrix(1,0,0,1,652.392,-0.400578)">
        <g transform="matrix(4.16667,0,0,4.16667,-735.553,0)">
          <g transform="matrix(0.24,0,0,0.24,0,0)">
            <path
              d="M134.44,316.535C145.027,441.531 249.98,539.829 377.711,539.829C474.219,539.829 557.724,483.712 597.342,402.371L645.949,419.197C599.165,520.543 496.595,590.954 377.711,590.954C221.751,590.954 93.869,469.779 83.161,316.535L134.44,316.535ZM83.946,265.645C99.012,116.762 224.88,0.401 377.711,0.401C540.678,0.401 672.987,132.71 672.987,295.677C672.987,321.817 669.583,347.168 663.194,371.313L614.709,354.529C619.381,335.689 621.862,315.971 621.862,295.677C621.862,160.926 512.461,51.526 377.711,51.526C253.133,51.526 150.223,145.03 135.392,265.645L83.946,265.645Z"
              style="fill: var(--color-brand)"
            />
          </g>
        </g>
      </g>
    </svg>
  </div>
</template>

<style lang="scss" scoped>
div {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 5rem;
  margin-top: 1rem;

  svg {
    width: 5rem;
    height: 5rem;
    position: absolute;
    &.rotate {
      animation: rotate 4s infinite linear;
      &.inner {
        animation: rotate 6s infinite linear reverse;
      }
    }
    @keyframes rotate {
      0% {
        transform: rotate(0);
      }
      100% {
        transform: rotate(360deg);
      }
    }
  }
}
</style>
