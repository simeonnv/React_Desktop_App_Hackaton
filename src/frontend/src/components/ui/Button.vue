<template>
    <button
      :class="buttonClasses"
      :disabled="disabled"
      @click="handleClick"
    >
      <slot></slot>
    </button>
  </template>
  
  <script lang="ts">
  export default {
    name: 'Button',
    props: {
      // Define props for customization
      variant: {
        type: String,
        default: 'primary', // Default button style
        validator: (value: string) => ['primary', 'secondary', 'danger', 'border'].includes(value), // Allowed variants
      },
      disabled: {
        type: Boolean,
        default: false,
      }
    },
    computed: {
      // Dynamically generate button classes based on props
      buttonClasses() {
        return [
            button_class,
            this.variant == 'primary' ? button_primary_class : "",
            this.variant == 'secondary' ? button_secondary_class : "",
            this.variant == 'danger' ? button_danger_class : "",
            this.variant == 'outline' ? button_outline_class : "",
            { 'button--disabled': this.disabled },
        ];
      },
    },
    methods: {
      handleClick() {
        if (!this.disabled) {
          this.$emit('click'); // Emit a click event if the button is not disabled
        }
      },
    },
  };

  const button_class = `
    transition-all
    transition-discrete
    ease-in-out
    duration-150
    transform
    hover:-translate-y-0.5
    flex w-full justify-center 
    rounded-md px-3 py-1.5
  `

  const button_primary_class = `
    bg-purple-600 
    text-sm/6 font-semibold 
    text-white shadow-xs 
    hover:bg-purple-700

    focus-visible:outline-2 
    focus-visible:outline-offset-2 
    focus-visible:outline-black-600
    focus:outline-2 focus:outline-offset-2 
    focus:outline-violet-500 active:bg-violet-800
  `

  const button_secondary_class = `
    bg-stone-700 
    text-sm/6 font-semibold 
    text-white shadow-xs 
    hover:bg-stone-600

    focus-visible:outline-2 
    focus-visible:outline-offset-2 
    focus-visible:outline-black-600
    focus:outline-2 focus:outline-offset-2 
    focus:outline-stone-600 active:bg-stone-500
  `

  const button_danger_class = `
    bg-red-700 
    text-sm/6 font-semibold 
    text-white shadow-xs 
    hover:bg-red-600

    focus-visible:outline-2 
    focus-visible:outline-offset-2 
    focus-visible:outline-black-600
    focus:outline-2 focus:outline-offset-2 
    focus:outline-red-600 active:bg-red-500
  `

  const button_outline_class = `
    text-sm/6 font-semibold 
    text-purple-600 shadow-xs
    
    hover:bg-gray-100

    focus-visible:outline-2 
    focus-visible:outline-offset-2 
    focus-visible:outline-purple-600
    focus:outline-2 focus:outline-offset-2 
    focus:outline-purple-600 active:bg-gray-200
  `

  </script>
  
 