import React, { useRef, useState } from "react";
import { CardBody, CardContainer, CardItem } from "../ui/3d-card";
import { TypewriterEffectSmooth } from "../ui/typewriter-effect";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { cn } from "../../lib/utils";
import { QueryClient, QueryClientProvider, useMutation } from "@tanstack/react-query";

const queryClient = new QueryClient();

const words = [
  {
    text: "Welcome",
    className: "dark:text-purple-700 text-purple-700",
  },
  {
    text: "back!",
    className: "dark:text-purple-700 text-purple-700"
  }
];

type Req = {
  username: string,
  password: string
}

type Res = {
  status: string,
  data: string // assuming this holds your token
}

const LoginUser = async ({ username, password }: Req): Promise<Res> => {
  const response = await fetch('http://localhost:6004/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password }),
  });

  if (!response.ok) {
    let data: Res | undefined = await response.json();
    if (data) {
      if (response.status === 400)
        throw new Error(`Login failed: invalid credential length`);
      throw new Error(`Login failed: ${data.status}`);
    } else {
      throw new Error(`Login failed`);
    }
  }
  
  return response.json();
};

export default function LoginCardRouter() {
  return (
    <QueryClientProvider client={queryClient}>
      <LoginCard/>
    </QueryClientProvider>
  );
}

export function LoginCard() {

  const usernameRef = useRef<string>("");
  const passwordRef = useRef<string>("");

  const { mutate, isPending, isError, error } = useMutation<Res, Error, Req>({
    mutationFn: LoginUser,
    onSuccess: (data) => {
        localStorage.setItem('token', data.data);
        document.cookie = `auth-token=${data.data}; path=/; secure; samesite=strict`;
        window.location.href = '/';
      },
    onError: (error) => {
      console.error('Signup error:', error);
    },
  });

  type PasswordError = {
    error: boolean,
    message: string
  }
  const [inputPasswordError, setInputPasswordError] = useState<PasswordError>({error: false, message: ""});

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const username = usernameRef.current;
    const password = passwordRef.current;

    if (!(password.length >= 5 && password.length < 30)) {
      setInputPasswordError({error: true, message: "Your credentials must be longer!"});
      return;
    }

    if (!(username.length >= 5 && username.length < 30)) {
      setInputPasswordError({error: true, message: "Your credentials must be longer!"});
      return;
    }

    mutate({ username, password });
  };

  return (
    <CardContainer className="gap-2">
      <CardBody className="bg-gray-50 flex flex-col gap-1 relative group/card dark:hover:shadow-2xl dark:hover:shadow-purple-500/[0.3] dark:bg-black dark:border-purple-500/[0.2] border-purple-500/[0.1] w-auto sm:w-[30rem] h-auto rounded-xl p-3 border-2">
        <CardItem translateZ="50" className="flex flex-row w-full justify-center align-middle items-center !text-xl font-bold !text-purple-500">
          <div>
            <TypewriterEffectSmooth words={words} className="text-purple-500 text-xl"/>
          </div>
        </CardItem>

        <CardItem as="div" translateZ="60" className="text-neutral-500 !w-full px-5 text-sm mt-2 dark:text-neutral-300">
          <LabelInputContainer className="pb-6 !w-full">
            <Label htmlFor="username">Username</Label>
            <Input onChange={(e) => (usernameRef.current = e.target.value)} id="username" className="!w-full" placeholder="Enter Username" type="text" />
          </LabelInputContainer>
        </CardItem>

        <CardItem as="div" translateZ="60" className="text-neutral-500 w-full text-sm px-5 mt-2 dark:text-neutral-300">
          <LabelInputContainer className="mb-4">
            <Label htmlFor="password">Confirm Password</Label>
            <Input onChange={(e) => (passwordRef.current = e.target.value)} id="password" placeholder="••••••••" type="password" />
          </LabelInputContainer>
          <div className="h-4">
            {isError && <p style={{ color: 'red' }}>{error.message}</p>}
            {inputPasswordError.error && <p style={{ color: 'red' }}>{inputPasswordError.message}</p>}
          </div>
        </CardItem>
        
        <div className="flex justify-between items-center mt-10">
          <CardItem translateZ={20} href="/login" target="__blank" className="px-4 py-2 rounded-xl text-xs font-normal dark:text-white dark:hover:text-gray-400">
            <a href="/signup">Are you new here?</a>
          </CardItem>
          <CardItem translateZ={20} as="button" onClick={handleSubmit} className="px-4 py-2 rounded-xl bg-black dark:bg-white dark:text-black text-white text-xs font-bold dark:hover:bg-gray-400" disabled={isPending}>
            {isPending ? 'Logging in...' : 'Login'}
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