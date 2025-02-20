import React, { useRef, useState } from "react";
import { CardBody, CardContainer, CardItem } from "../ui/3d-card";
import { TypewriterEffectSmooth } from "../ui/typewriter-effect";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { cn } from "../../lib/util/utils";
import { QueryClient, QueryClientProvider, useMutation, useQuery } from "@tanstack/react-query";

const queryClient = new QueryClient();

const words = [
  {
    text: "Are",
    className: "dark:text-purple-700 text-purple-700",
  },
  {
    text: "you",
    className: "dark:text-purple-700 text-purple-700",
  },
  {
    text: "new",
    className: "dark:text-purple-700 text-purple-700",
  },
  {
    text: "here?",
    className: "dark:text-purple-700 text-purple-700"
  }
];

type Req = {
  username: string,
  password: string
}

type Res = {
  status: string,
  data: string
}

const SignupUser = async ({ username, password }: Req): Promise<Res> => {
  const response = await fetch('http://localhost:6004/auth/signup', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password }),
  });

  if (!response.ok) {
    let data: Res | undefined = await response.json()
    if (data) {
      if (response.status == 400)
        throw new Error(`Signup failed: invalid credenetial lenght`);
      throw new Error(`Signup failed: ${data.status}`);
    } else {
      throw new Error(`Signup failed`);
    }
  }
    

  return response.json();
};

export default function SignupCardRouter() {
  return (
    <QueryClientProvider client={queryClient}>
      <SignupCard/>
    </QueryClientProvider>
  )
}


export function SignupCard() {

  const usernameRef = useRef<string>("");
  const passwordRef = useRef<string>("");
  const repeatPasswordRef = useRef<string>("");

  type PasswordError = {
    error: boolean,
    message: string
  }
  const [inputPasswordError, setInputPasswordError] = useState<PasswordError>({error: false, message: ""});

  const { mutate, isPending, isError, error } = useMutation<Res, Error, Req>({
    mutationFn: SignupUser,
    onSuccess: (data) => {
      console.log('Signup successful!', data);
      alert('Signup successful!');
      // Redirect or update state here
    },
    onError: (error) => {
      console.error('Signup error:', error);
    },
  });


  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const username = usernameRef.current
    const password = passwordRef.current

    if (!(password.length >= 5 && password.length < 30)) {
      setInputPasswordError({error: true, message: "Your credentials must be longer!"})
      return
    }

    if (!(username.length >= 5 && username.length < 30)) {
      setInputPasswordError({error: true, message: "Your credentials must be longer!"})
      return
    }


    if (password === repeatPasswordRef.current) {
      setInputPasswordError({error: false, message: ""})
      
      mutate({ username, password });
    }
    else
      setInputPasswordError({error: true, message: "You misspelled your repeated password!"})
  };

  return (
      <CardContainer className="gap-2">
        <CardBody className="bg-gray-50 flex flex-col gap-1 relative group/card dark:hover:shadow-2xl dark:hover:shadow-purple-500/[0.3] dark:bg-black dark:border-purple-500/[0.2] border-purple-500/[0.1] w-auto sm:w-[30rem] h-auto rounded-xl p-3 border-2  ">
          <CardItem translateZ="50" className="flex flex-row w-full justify-center align-middle items-center !text-xl font-bold !text-purple-500 ">
            <div><TypewriterEffectSmooth words={words} className="text-purple-500 text-xl"/></div>
          </CardItem>

          <CardItem as="p" translateZ="60" className="text-neutral-500 !w-full px-5 text-sm mt-2 dark:text-neutral-300">
            <LabelInputContainer className=" pb-6 !w-full">
              <Label htmlFor="username">Username</Label>
              <Input onChange={(e) => (usernameRef.current = e.target.value)} id="username" className="!w-full" placeholder="Enter Username" type="text" />
            </LabelInputContainer>
          </CardItem>

          <CardItem as="p" translateZ="60" className="text-neutral-500 !w-full px-5 text-sm mt-2 dark:text-neutral-300">
            <LabelInputContainer className="mb-4">
              <Label htmlFor="password">Password</Label>
              <Input onChange={(e) => (passwordRef.current = e.target.value)} id="password" placeholder="••••••••" type="password" />
            </LabelInputContainer>
          </CardItem>

          <CardItem as="p" translateZ="60" className="text-neutral-500 w-full text-sm px-5 mt-2 dark:text-neutral-300">
            <LabelInputContainer className="mb-4">
              <Label htmlFor="password">Confirm Password</Label>
              <Input onChange={(e) => (repeatPasswordRef.current = e.target.value)} id="password" placeholder="••••••••" type="password" />
            </LabelInputContainer>
            <div className="h-4">
            {isError && <p style={{ color: 'red' }}>{error.message}</p>}
            {inputPasswordError.error && <p style={{ color: 'red' }}>{inputPasswordError.message}</p>}
            </div>
          </CardItem>
          
          <div className="flex justify-between items-center mt-10">
            <CardItem translateZ={20} href="/login" target="__blank" className="px-4 py-2 rounded-xl text-xs font-normal dark:text-white dark:hover:text-gray-400">
              <a href="/login">Have an account?</a>
            </CardItem>
            <CardItem translateZ={20} as="button" onClick={handleSubmit} className="px-4 py-2 rounded-xl bg-black dark:bg-white dark:text-black text-white text-xs font-bold dark:hover:bg-gray-400" disabled={isPending}>
              {isPending ? 'Signing up...' : 'Signup'}
            </CardItem>
          </div>
        </CardBody>
      </CardContainer>
  );
}


const LabelInputContainer = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => {
  return (
    <div className={cn("flex flex-col space-y-2 w-full", className)}>
      {children}
    </div>
  );
};