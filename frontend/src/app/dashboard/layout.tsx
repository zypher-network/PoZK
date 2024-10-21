'use client';
import Navbar from "@/components/widget/navbar/navbar";
import Header from "@/components/widget/Header/Header";
import useAuth from "@/components/hooks/useAuth";
import useGetUserBalance from "@/components/hooks/useGetUserBalance";
import useInitSubgraph from "@/components/hooks/useInitSubgraph";

export default function DashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  useInitSubgraph();
  useGetUserBalance();
  const [hasAuth] = useAuth();
  console.log(hasAuth, '---');
  if (!hasAuth) {
    return (
      <div className="grid place-items-center animate-pulse p-4">Loading</div>
    )
  }
  return (
    <div className="overflow-y-scroll flex flex-row right-0 left-0 top-0 bottom-0">
      <div className="h-screen fixed left-0 top-0 bottom-0 w-[288px] bg-background z-40">
        <Navbar />
      </div>
      {/* min-w-[1400px]  */}
      <main className="w-full h-full flex flex-col ml-[300px] mr-[40px]">
        <Header />
        <div className="ml-18">{children}</div>
      </main>
    </div>
  );
}
