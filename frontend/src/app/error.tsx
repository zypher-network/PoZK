"use client";
interface ErrorProps {
  statusCode?: number;
  message?: string;
}
const ErrorPage: React.FC<ErrorProps> = ({ statusCode, message }) => {
  return (
    <div className="grid min-h-full place-items-center px-6 py-24 sm:py-32 lg:px-8 text-center">
      <p className="text-base font-semibold text-indigo-600">Error</p>
      <h1 className="mt-4 text-3xl font-bold tracking-tight sm:text-5xl">
        {statusCode}
      </h1>
      <p className="mt-6 text-base leading-7">
        An error occurred while processing your request.
      </p>
      <div className="mt-10 flex items-center justify-center gap-x-6">
        <a
          href="#"
          className="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        >
          Go back home
        </a>
      </div>
    </div>
  );
};

export default ErrorPage;
